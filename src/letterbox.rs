use libc::{getpid, pid_t, sem_post, uintptr_t};

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{SHM_LETTERBOX_NAME, SHM_SEMAPHORE_NAME};

// todo: depending on feature set controller, measuring method, and num samples
type Controller = crate::EnergyController;
//type Controller = crate::RuntimeController;

/// A letterbox is a hashmap-like mapping from unique identifiers (function
/// pointers) to incoming (runtime/energy measurements) and outgoing
/// (thread-count) data.
#[repr(C)]
pub struct Letterbox {
    len: usize,
    pub buckets: [Bucket<20>; 64],
}

#[repr(C)]
pub enum Bucket<const NUM_SAMPLES: usize> {
    Empty,
    Occupied(pid_t, uintptr_t, Controller, Incoming<NUM_SAMPLES>, i32),
    Tombstone,
}

#[repr(C)]
pub struct Incoming<const NUM_SAMPLES: usize> {
    pub len: usize,
    pub data: [f32; NUM_SAMPLES],
}

#[no_mangle]
unsafe extern "C" fn MTD_letterbox_open() -> *mut Letterbox {
    use libc::{shm_open, O_RDWR, S_IRUSR, S_IWUSR};
    println!("opening letterbox");
    let fd = shm_open(SHM_LETTERBOX_NAME, O_RDWR, (S_IRUSR | S_IWUSR) as u32);
    if fd < 0 {
        eprintln!("resource controller is not running");
        std::ptr::null_mut()
    } else {
        Letterbox::from_mmap(fd)
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_letterbox_push(lb: &mut Letterbox, key: uintptr_t, value: f32) {
    let pid = unsafe { getpid() };

    println!("push {:?} = {}", key, value);
    if let Some(incoming) = lb.get_incoming_mut(key) {
        assert!(incoming.len < 20);
        incoming.data[incoming.len] = value;
        incoming.len += 1;

        if incoming.len == 20 {
            let sem = libc::sem_open(SHM_SEMAPHORE_NAME, 0);
            assert_ne!(sem, std::ptr::null_mut());
            let res = sem_post(sem);
            assert_eq!(res, 0);
            incoming.len = 0;
        }
    } else {
        println!("pushing new fptr {}", key);
        assert!(lb.len < lb.buckets.len());
        lb.insert(pid, key, value);
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_thread_count(lb: &mut Letterbox, key: uintptr_t) -> u32 {
    if let Some(thread_count) = lb.get_threads(key) {
        *thread_count as u32
    } else {
        16
    }
}

#[no_mangle]
unsafe extern "C" fn MTD_free_pid(lb: &mut Letterbox) {
    let pid = unsafe { getpid() };
    println!("Freeing letterboxes of {}", pid);

    for bucket in lb.buckets.iter_mut() {
        match bucket {
            Bucket::Occupied(pid2, fptr, ..) if pid == *pid2 => {
                println!("Cleaning {}:{}", pid, fptr);
                *bucket = Bucket::Tombstone;
                lb.len -= 1;
            }
            _ => { },
        }
    }
}

impl Letterbox {
    pub unsafe fn from_mmap<'a>(fd: i32) -> &'a mut Self {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            std::mem::size_of::<Self>(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0
        );
        assert_ne!(ptr, std::ptr::null_mut());
        &mut *(ptr as *mut Self)
    }

    pub fn insert(&mut self, pid: pid_t, key: uintptr_t, value: f32) {
        assert!(self.len < self.buckets.len());
        let start_idx = self.get_hash(key);

        let (lhs, rhs) = self.buckets.split_at_mut(start_idx);

        for bucket in rhs.iter_mut().chain(lhs.iter_mut()) {
            match bucket {
                Bucket::Empty |
                Bucket::Tombstone => {
                    println!("found a spot for {}", key);
                    let mut data = [0.0; 20];
                    data[0] = value;
                    *bucket = Bucket::Occupied(
                        pid,
                        key,
                        Controller::new(16),
                        Incoming { len: 1, data },
                        16
                    );
                    println!("inserted {}", key);
                    return;
                }
                _ => { },
            }
        }
    }

    pub fn get_threads(&self, key: uintptr_t) -> Option<&i32> {
        let start_idx = self.get_hash(key);

        let (lhs, rhs) = self.buckets.split_at(start_idx);

        for bucket in rhs.iter().chain(lhs.iter()) {
            match bucket {
                Bucket::Empty => return None,
                Bucket::Occupied(_, k, _, _, o) if key == *k => return Some(o),
                _ => { },
            }
        }

        None
    }

    pub fn get_incoming_mut(&mut self, key: uintptr_t) -> Option<&mut Incoming<20>> {
        let start_idx = self.get_hash(key);

        let (lhs, rhs) = self.buckets.split_at_mut(start_idx);

        for bucket in rhs.iter_mut().chain(lhs.iter_mut()) {
            match bucket {
                Bucket::Empty => return None,
                Bucket::Occupied(_, k, _, incoming, _) if key == *k => return Some(incoming),
                _ => { },
            }
        }

        None
    }

    fn get_hash(&self, key: uintptr_t) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }
}
