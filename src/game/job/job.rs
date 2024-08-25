use crate::room::room::ChatType;

use super::option::{JobOption, JobStatus};
use super::super::event::Event;

// trait Job : 게임 내에서의 직업을 가리킨다.
// 사용자의 입력을 받으면 우선 Job 객체로 전달되며, 
// Job 객체는 Vec<Event>를 반환해야 한다.
// Job 객체에서 트리거된 이벤트는 메인 로직에서 전부 처리된다.

// 채팅은 예외적으로 클로저를 반환한다
pub trait Job {
    // option(): 이 직업의 메타데이터를 반환한다.
    // 다른 직업으로 인한 능력 무효 (ex. 마담) 과는 별개로, 메타데이터는 변하지 않아야 한다.
    fn option(&self) -> JobOption;

    // 현재 나의 상태를 반환한다.
    fn status<'a>(&'a mut self) -> &'a mut JobStatus;

    // 유동 손 직업일 경우 호출된다
    fn is_valid_hand(&mut self, players: &Vec<Box<dyn Job>>, idx: usize) -> bool;

    // 고정 손 직업일 경우 플레이어 지목 직후에, 그렇지 않을 경우 낮이 될 때 호출된다.
    fn hand(&self, players: &Vec<Box<dyn Job>>, idx: usize) -> Vec<Event>;

    // 마피아에게 지목당했을 때 호출된다.
    // 여기서 의사의 치료처리, 군인의 방탄처리 등등을 한다.
    fn on_got_murderred(&self, players: &Vec<Box<dyn Job>>, idx: usize) -> Vec<Event>;

    // 채팅을 하면 호출된다.
    // 반환값은 함수인데, 채팅을 받을 사람을 지정하는 함수를 호출한다.
    // 반환값 함수가 true를 리턴하는 사람들에게만 채팅이 전달된다.
    fn chat(&self, message: String) -> Box<dyn Fn(&Box<dyn Job>) -> ChatType>;
}