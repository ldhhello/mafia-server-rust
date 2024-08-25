use std::fmt::{Display, Formatter};

use job::Job;
use mafia::mafia::Mafia;

pub mod job;
pub mod option;

pub mod mafia;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JobList {
    None,
    Mafia, Police, Doctor, Special, Assist, Cult,
    Citizen, Soldier, Politician, Shaman, Reporter,
    Gangster, Detective, Ghoul, Terrorist, Priest,
    Magician, Hacker, Prophet, Judge, Nurse,
    Mentalist, Couple, 
    Villain, Spy, Beastman, Madam, Thief,
    Scientist, Witch, Frog,

    // 기존 C++ 구현체가 Citizen이랑 Villain 을 기준으로 특직이랑 보조직을 구분하는데
    // 이 방법은 별로 좋은 생각이 아니다.
    // 앞으로 새로 만드는 직업들은 그냥 저 직업 리스트에 뒤쪽부터 채울 예정
    // 근데 신직업들 영문 이름이 죄다 어렵네..

    Swindler /* 사기꾼 */, HitMan /* 청부업자 */,
    Agent /* 요원 */, Mercenary /* 용병 */,
    Administrator /* 공무원 */, Cabal /* 비밀결사 */,
    Paparazzi /* 파파라치 */,
    Fanatic /* 광신도 */
}

impl JobList {
    pub fn create_job(self) -> Box<dyn Job + Send> {
        match self {
            Self::Mafia => Box::new(Mafia::new()),
            _ => Box::new(Mafia::new())
        }
    }
}

impl Default for JobList {
    fn default() -> Self { JobList::None }
}

impl Display for JobList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match *self {
            JobList::None => "무직",
            JobList::Mafia => "마피아",
            JobList::Police => "경찰",
            JobList::Doctor => "의사",
            JobList::Special => "특수 직업",
            JobList::Assist => "보조 직업",
            JobList::Cult => "교주",
            JobList::Citizen => "시민",
            JobList::Soldier => "군인",
            JobList::Politician => "정치인",
            JobList::Shaman => "영매",
            JobList::Reporter => "기자",
            JobList::Gangster => "건달",
            JobList::Detective => "사립탐정",
            JobList::Ghoul => "도굴꾼",
            JobList::Terrorist => "테러리스트",
            JobList::Priest => "성직자",
            JobList::Magician => "마술사",
            JobList::Hacker => "해커",
            JobList::Prophet => "예언자",
            JobList::Judge => "판사",
            JobList::Nurse => "간호사",
            JobList::Mentalist => "심리학자",
            JobList::Couple => "연인",
            JobList::Villain => "악인",
            JobList::Spy => "스파이",
            JobList::Beastman => "짐승인간",
            JobList::Madam => "마담",
            JobList::Thief => "도둑",
            JobList::Scientist => "과학자",
            JobList::Witch => "마녀",
            JobList::Frog => "개구리",
            JobList::Swindler => "사기꾼",
            JobList::HitMan => "청부업자",
            JobList::Agent => "요원",
            JobList::Mercenary => "용병",
            JobList::Administrator => "공무원",
            JobList::Cabal => "비밀결사",
            JobList::Paparazzi => "파파라치",
            JobList::Fanatic => "광신도",
        });
        Ok(())
    }
}

pub enum Team {
    MafiaTeam,
    CitizenTeam,
    CultTeam
}