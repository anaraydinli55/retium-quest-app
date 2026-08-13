use std::collections::HashMap; // Standart HashMap yapısını içe aktardık!
use std::vec::Vec;
use std::string::String;

// Retium RCP-1 Standartına uyğun istifadəçi profili
pub struct UserProfile {
    pub username: String,
    pub experience_points: u64,
    pub completed_quests: Vec<u32>,
    pub earned_badges: Vec<String>,
}

// Müqavilənin daxili depolama (Storage) sahəsi
pub struct RetiumQuestStorage {
    // İstifadəçi ünvanlarını (Address/String) profillərə bağlayır
    pub user_profiles: HashMap<String, UserProfile>,
    // Quest ID-lərini mükafat olaraq veriləcək XP miqdarına bağlayır
    pub quest_rewards: HashMap<u32, u64>,
}

impl RetiumQuestStorage {
    /// Yeni bir boş depolama sahəsi ilkləndirir.
    pub fn new() -> Self {
        let mut quest_rewards = HashMap::new();
        // Varsayılan Questləri və XP mükafatlarını təyin edirik
        quest_rewards.insert(1, 100); // Quest 1: Miden-də ilk transfer (100 XP)
        quest_rewards.insert(2, 250); // Quest 2: Banka depozit (250 XP)
        quest_rewards.insert(3, 500); // Quest 3: ZK-AMM Swap (500 XP)

        Self {
            user_profiles: HashMap::new(),
            quest_rewards,
        }
    }

    /// Yeni bir istifadəçini "Retium Quest Portalına" qeydiyyatdan keçirir.
    pub fn register_user(&mut self, user_address: String, username: String) -> bool {
        if self.user_profiles.contains_key(&user_address) {
            return false; // İstifadəçi artıq qeydiyyatdan keçib
        }

        let profile = UserProfile {
            username,
            experience_points: 0,
            completed_quests: Vec::new(),
            earned_badges: Vec::new(),
        };

        self.user_profiles.insert(user_address, profile);
        true
    }

    /// İstifadəçinin müvafiq quest-i tamamladığını təsdiqləyir,
    /// onun XP-sini artırır və əgər limitləri keçibsə on-chain Soulbound RCP-1 nişanını (badge) hədiyyə edir.
    pub fn complete_quest(&mut self, user_address: String, quest_id: u32) -> Result<u64, &'static str> {
        // 1. Doğrulama: İstifadəçi sistemdə qeydiyyatda olmalıdır
        let profile = self.user_profiles.get_mut(&user_address)
            .ok_or("Istifadeci profili tapilmadi! Zehmet olmasa qeydiyyatdan kecin.")?;

        // 2. Doğrulama: Bu quest daha əvvəl tamamlanmamış olmalıdır (double-claiming qarşısını alırıq)
        if profile.completed_quests.contains(&quest_id) {
            return Err("Bu tapshiriq daha evvel tamamlanib!");
        }

        // 3. Doğrulama: Quest mövcud olmalıdır
        let xp_reward = self.quest_rewards.get(&quest_id)
            .ok_or("Muvafiq Quest ID tapilmadi!")?;

        // Quest-i tamamlananlar siyahısına əlavə et
        profile.completed_quests.push(quest_id);
        
        // İstifadəçinin ümumi XP-sini artır
        profile.experience_points += *xp_reward;

        // 4. RCP-1 Reputation / Badge Sistemi: Müvafiq XP limitlərinə görə on-chain Soulbound Badge verilir
        if profile.experience_points >= 100 && !profile.earned_badges.contains(&String::from("Bronze Explorer")) {
            profile.earned_badges.push(String::from("Bronze Explorer"));
        }
        if profile.experience_points >= 350 && !profile.earned_badges.contains(&String::from("Silver Builder")) {
            profile.earned_badges.push(String::from("Silver Builder"));
        }
        if profile.experience_points >= 850 && !profile.earned_badges.contains(&String::from("Gold ZK-Master")) {
            profile.earned_badges.push(String::from("Gold ZK-Master"));
        }

        Ok(profile.experience_points)
    }

    /// İstifadəçinin ümumi XP miqdarını geri qaytarır.
    pub fn get_user_xp(&self, user_address: String) -> u64 {
        match self.user_profiles.get(&user_address) {
            Some(profile) => profile.experience_points,
            None => 0,
        }
    }

    /// İstifadəçinin qazandığı on-chain RCP-1 nişanlarının siyahısını geri qaytarır.
    pub fn get_user_badges(&self, user_address: String) -> Vec<String> {
        match self.user_profiles.get(&user_address) {
            Some(profile) => profile.earned_badges.clone(),
            None => Vec::new(),
        }
    }
}

// -------------------------------------------------------------
// Rəsmi RCP-1 Quest Sistemi - Lokal Unit Test Bloğu
// -------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_system_flow() {
        // 1. Müqavilə deposunu başladırıq
        let mut storage = RetiumQuestStorage::new();
        
        let user_address = String::from("mtst1aqt64yrkd46khqfe32dlp02gtcpeglvy_qr7qqq9wr6w");
        let username = String::from("AzAgent");

        // 2. İstifadəçinin uğurla qeydiyyatdan keçdiyini yoxlayırıq
        assert!(storage.register_user(user_address.clone(), username.clone()));
        
        // 3. Təkrar qeydiyyatın təhlükəsiz şəkildə rədd edildiyini yoxlayırıq
        assert!(!storage.register_user(user_address.clone(), username.clone()));

        // 4. Miden-də ilk transfer questini (Quest 1 - 100 XP) tamamlayırıq
        let xp_after_q1 = storage.complete_quest(user_address.clone(), 1).unwrap();
        assert_eq!(xp_after_q1, 100);
        
        // "Bronze Explorer" nişanının qazanıldığını yoxlayırıq
        let badges_after_q1 = storage.get_user_badges(user_address.clone());
        assert!(badges_after_q1.contains(&String::from("Bronze Explorer")));

        // 5. Eyni questin təkrar tamamlanmasının təhlükəsiz rədd edildiyini yoxlayırıq
        let duplicate_result = storage.complete_quest(user_address.clone(), 1);
        assert!(duplicate_result.is_err());

        // 6. Banka depozit questini (Quest 2 - 250 XP) tamamlayırıq
        let xp_after_q2 = storage.complete_quest(user_address.clone(), 2).unwrap();
        assert_eq!(xp_after_q2, 350); // 100 + 250 = 350 XP
        
        // "Silver Builder" nişanının qazanıldığını yoxlayırıq
        let badges_after_q2 = storage.get_user_badges(user_address.clone());
        assert!(badges_after_q2.contains(&String::from("Silver Builder")));

        // 7. ZK-AMM Swap questini (Quest 3 - 500 XP) tamamlayırıq
        let xp_after_q3 = storage.complete_quest(user_address.clone(), 3).unwrap();
        assert_eq!(xp_after_q3, 850); // 350 + 500 = 850 XP
        
        // "Gold ZK-Master" nişanının qazanıldığını yoxlayırıq
        let badges_after_q3 = storage.get_user_badges(user_address.clone());
        assert!(badges_after_q3.contains(&String::from("Gold ZK-Master")));

        // 8. Son bakiye və nişan siyahısını yoxlayırıq
        assert_eq!(storage.get_user_xp(user_address.clone()), 850);
        assert_eq!(storage.get_user_badges(user_address.clone()).len(), 3);
        
        println!("SUCCESS: All RCP-1 Quest System flows passed natively!");
    }
}