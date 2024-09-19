use std::{collections::BinaryHeap, sync::Arc};
use std::cmp::Reverse;
use tokio::sync::Mutex;

use super::room::{Room, RoomInfo, RoomOption};

// 방 id 번호를 관리한다
// 요청이 들어오면 최대한 낮은 번호를 할당시켜준다!
// 방번호는 0번부터 시작한다 (마피아42가 이렇게 작동함)
pub struct IdManager {
    heap: BinaryHeap<Reverse<i32>>,
    len: usize
}

impl IdManager {
    pub fn new() -> IdManager {
        IdManager {
            heap: BinaryHeap::new(),
            len: 0
        }
    }
    pub fn generate(&mut self) -> i32 {
        return match self.heap.pop() {
            Some(k) => k.0,
            None => {
                let res = self.len as i32;
                self.len += 1;
                res
            }
        };
    }
    // 이 함수는 잘못된 데이터가 들어오면 어떻게 돌아갈 지 모른다!
    // 알아서 잘 사용해야 한다.
    pub fn free(&mut self, id: i32) {
        self.heap.push(Reverse(id));
    }
}

pub struct RoomManager {
    id_manager: Mutex<IdManager>,
    rooms: Vec<Mutex<Option<Arc<Room>>>>
}

impl RoomManager {
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(2000);
        for _ in 0..2000 {
            vec.push(Mutex::new(None));
        }
        return Self {
            id_manager: Mutex::new(IdManager::new()),
            rooms: vec
        }
    }
    pub async fn list(&self) -> Vec<RoomInfo> {
        // 이런 연산들은 mutex 때매 함수형 틱하게 할 방법이 마땅하지 않은 것 같다. ㅠㅠ
        let mut res = Vec::new();

        // 이렇게 돌리면 한 번 요청 마다 2000회 연산을 해야 한다.
        // 나중에 효율적으로 바꿀 예정
        for room in &self.rooms {
            let room = room.lock().await;
            let Some(room) = room.as_ref() else { continue };

            let info = room.info().await;
            if info.now_people > 0 {
                res.push(info);
            }
        }

        return res;
    }
    // 방 생성 후 방 id 반환
    pub async fn create(self: &Arc<Self>, option: RoomOption) -> Arc<Room> {
        let id = self.id_manager.lock().await.generate();

        let room = Arc::new(Room::new(self.clone(), id, option));

        *self.rooms[id as usize].lock().await = Some(room.clone());

        return room;
    }
    pub async fn get(&self, id: i32) -> Option<Arc<Room>> {
        self.rooms[id as usize].lock().await.clone()
    }
    pub async fn delete(self: &Arc<Self>, id: i32) {
        *self.rooms[id as usize].lock().await = None;

        self.id_manager.lock().await.free(id);
    }
}