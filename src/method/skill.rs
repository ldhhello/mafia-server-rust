pub const MAFIA_KILL: i32 = 0;
pub const NOTHING_HAPPENED: i32 = 1;
pub const CAUGHT_MAFIA: i32 = 2;
pub const NO_MAFIA: i32 = 3;
pub const DOCTOR_HEAL: i32 = 4;
pub const SEONGBUL: i32 = 5;
pub const GOT_SEONGBULLED: i32 = 6;
pub const HEADLINE: i32 = 7;
pub const HEADLINE_FAILED: i32 = 8;
pub const SOLDIER_BLOCK: i32 = 9;
pub const GANGSTER_THREATEN: i32 = 10;
pub const GOT_THREATENED: i32 = 11;
pub const GRAVE_ROB: i32 = 12;
pub const GOT_ROBBED: i32 = 13;
pub const POLITICIAN: i32 = 14;
pub const JOB: i32 = 15;
pub const CONNECT: i32 = 16;
pub const EATEN_BY_BEAST: i32 = 17;
pub const REVIVE: i32 = 18;
pub const REVIVE_FAILED: i32 = 19;
pub const TERRORIST_MAFIA_EXPLODE: i32 = 20;
pub const TERRORIST_VOTE_EXPLODE: i32 = 21;
pub const GOT_TEMPTED: i32 = 22;
pub const MADAM_TEMPT: i32 = 23;
pub const THIEF_STEAL_SOLDIER: i32 = 24;
pub const THIEF_STEAL: i32 = 25;
pub const SOLDIER_BLOCK_THIEF: i32 = 26;
pub const DETECTIVE: i32 = 27;
pub const CULT_RECRUIT: i32 = 28;
pub const CULT_RECRUIT_FAILED: i32 = 29;
pub const PRIEST_BLOCK_CULT: i32 = 30;
pub const COUPLE_SACRIFICE: i32 = 31;
pub const SOLDIER_BLOCK_SPY: i32 = 32;
pub const SPY_GOT_DISCOVERED: i32 = 33;
pub const BECAME_FROG: i32 = 34;
pub const PROPHET_WIN: i32 = 35;
pub const JUDGE: i32 = 36;
pub const NURSE_CONNECT_DOCTOR: i32 = 37;
pub const DOCTOR_CONNECT_NURSE: i32 = 38;
pub const GOT_CULTED: i32 = 39;
pub const CULT_RINGS: i32 = 40;

pub const SKILL_STRINGS: [&str; 41] = [
    "%s님이 살해당했습니다.",
    "밤동안 아무 일도 일어나지 않았습니다.",
    "%s님은 마피아입니다!",
    "%s님은 마피아가 아닙니다.",
    "%s님이 의사의 치료를 받고 살아났습니다!",
    "성불 결과 %s님은 %s입니다!",
    "영매에게 성불당했습니다.",
    "특종입니다! %s님이 %s (이)라는 소식입니다!",
    "취재에 실패했습니다.",
    "군인 %s님이 공격을 버텨냈습니다!",
    "%s님에게 위협을 가했습니다.",
    "의문의 괴한에게 협박을 당했습니다.",
    "%s 직업을 획득했습니다!",
    "도굴꾼에게 도굴당해 %s이 되었습니다.",
    "%s은 투표로 죽지 않습니다.",
    "%s님의 직업은 %s입니다!",
    "접선했습니다.",
    "%s님이 짐승에게 습격당했습니다.",
    "%s님이 부활했습니다!",
    "부활에 실패했습니다.",
    "테러리스트 %s님이 마피아 %s님을 습격했습니다!",
    "테러리스트 %s님이 %s님과 함께 자폭했습니다!",
    "다른 플레이어에게 유혹당했습니다!",
    "플레이어 %s님을 유혹했습니다.",
    "훔치는 데 실패했습니다!",
    "%s님의 직업 %s (을)를 훔쳤습니다!",
    "도둑 %s님이 당신의 직업을 훔치려고 시도했습니다!",
    "%s님을 조사합니다.",
    "%s님을 포교했습니다.",
    "%s %s님을 포교하는 데 실패했습니다.",
    "교주 %s님이 당신을 포교하려고 시도했습니다!",
    "연인 %s님이 %s님 대신 희생했습니다!",
    "스파이 %s님이 당신을 조사하려고 시도했습니다.",
    "군인 %s님에게 당신의 정체를 들켰습니다!",
    "마녀에게 저주당해 개구리가 되었습니다.",
    "예언자 %s님의 힘으로 %s팀이 승리했습니다!",
    "판사 %s님이 투표 결과를 정했습니다.",
    "의사 %s님과 접선했습니다!",
    "간호사 %s님과 접선했습니다!",
    "교주 %s님에게 포교당했습니다.",
    "교주의 종소리가 울려퍼졌습니다."
];