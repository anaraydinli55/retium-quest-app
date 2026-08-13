use std::collections::HashMap; // Standart HashMap yapısını içe aktardık
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
    // 1. Quest ve Profil Depolama Alanları
    pub user_profiles: HashMap<String, UserProfile>,
    pub quest_rewards: HashMap<u32, u64>,

    // 2. RCP-1 NFT Toplu Basım Depolama Alanları
    // NFT Serial ID (u32) değerini, sahibinin adresine bağlar
    pub nft_owners: HashMap<u32, String>,
    // Toplam basılan (mint edilen) NFT sayısını tutar
    pub nft_supply: u32,
}

impl RetiumQuestStorage {
    /// Yeni bir boş depolama sahəsi ilkləndirir.
    pub fn new() -> Self {
        let mut quest_rewards = HashMap::new();
        // Varsayılan Questləri ve XP mükafatlarını təyin edirik
        quest_rewards.insert(1, 100); // Quest 1: Miden-də ilk transfer (100 XP)
        quest_rewards.insert(2, 250); // Quest 2: Banka depozit (250 XP)
        quest_rewards.insert(3, 500); // Quest 3: ZK-AMM Swap (500 XP)

        Self {
            user_profiles: HashMap::new(),
            quest_rewards,
            nft_owners: HashMap::new(),
            nft_supply: 0,
        }
    }

    /// Yeni bir istifadəçini "Retium Quest Portalına" qeydiyyatdan keçirir.
    pub fn register_user(&mut self, user_address: String, username: String) -> bool {
        if self.user_profiles.contains_key(&user_address) {
            return false;
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

    /// İstifadəçinin müvafiq quest-i tamamladığını təsdiqləyir və XP/Badge verir.
    pub fn complete_quest(&mut self, user_address: String, quest_id: u32) -> Result<u64, &'static str> {
        let profile = self.user_profiles.get_mut(&user_address)
            .ok_or("Istifadeci profili tapilmadi!")?;

        if profile.completed_quests.contains(&quest_id) {
            return Err("Bu tapshiriq daha evvel tamamlanib!");
        }

        let xp_reward = self.quest_rewards.get(&quest_id)
            .ok_or("Muvafiq Quest ID tapilmadi!")?;

        profile.completed_quests.push(quest_id);
        profile.experience_points += *xp_reward;

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

    // -------------------------------------------------------------
    // RCP-1 DOĞAL TOPLU NFT BASIM (BATCH MINT) FONKSİYONLARI
    // -------------------------------------------------------------

    /// Tek bir işlemde, tek bir onay ve sabit gaz ücretiyle 100 adede kadar NFT basar!
    pub fn batch_mint_nfts(&mut self, recipient_address: String, amount_to_mint: u32) -> Result<Vec<u32>, &'static str> {
        if amount_to_mint == 0 {
            return Err("Basılacak miktar sifir olamaz!");
        }

        // Retium limitlerini kontrol ediyoruz (Atıf işareti temizlendi)
        if amount_to_mint > 100 {
            return Err("Retium doğal toplu basim limiti tək bir tranzaksiyada maksimum 100 adettir!");
        }

        let mut minted_ids = Vec::new();

        for _ in 0..amount_to_mint {
            self.nft_supply += 1;
            let new_nft_id = self.nft_supply;
            
            self.nft_owners.insert(new_nft_id, recipient_address.clone());
            minted_ids.push(new_nft_id);
        }

        Ok(minted_ids)
    }

    /// Belirtilen NFT ID'sinin güncel sahibinin adresini sorgular.
    pub fn get_nft_owner(&self, nft_id: u32) -> Option<String> {
        self.nft_owners.get(&nft_id).cloned()
    }

    /// Toplam basılmış olan NFT miktarını (arzını) sorgular.
    pub fn get_nft_supply(&self) -> u32 {
        self.nft_supply
    }

    pub fn get_user_xp(&self, user_address: String) -> u64 {
        match self.user_profiles.get(&user_address) {
            Some(profile) => profile.experience_points,
            None => 0,
        }
    }

    pub fn get_user_badges(&self, user_address: String) -> Vec<String> {
        match self.user_profiles.get(&user_address) {
            Some(profile) => profile.earned_badges.clone(),
            None => Vec::new(),
        }
    }
}

// -------------------------------------------------------------
// Rəsmi RCP-1 Quest Sistemi & Toplu NFT - Lokal Unit Test Bloğu
// -------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_and_batch_nft_mint_flow() {
        let mut storage = RetiumQuestStorage::new();
        
        let user_address = String::from("mtst1aqt64yrkd46khqfe32dlp02gtcpeglvy_qr7qqq9wr6w");
        let username = String::from("AzAgent");

        assert!(storage.register_user(user_address.clone(), username.clone()));

        let xp_after_q1 = storage.complete_quest(user_address.clone(), 1).unwrap();
        assert_eq!(xp_after_q1, 100);

        // -------------------------------------------------------------
        // RCP-1 BATCH NFT MINT (TOPLU BASIM) TESTLERİ
        // -------------------------------------------------------------

        let mint_result = storage.batch_mint_nfts(user_address.clone(), 100);
        assert!(mint_result.is_ok(), "100 adet toplu NFT basimi basarisiz oldu!");
        
        let minted_nfts = mint_result.unwrap();
        assert_eq!(minted_nfts.len(), 100);
        assert_eq!(storage.get_nft_supply(), 100);

        assert_eq!(storage.get_nft_owner(1).unwrap(), user_address);
        assert_eq!(storage.get_nft_owner(100).unwrap(), user_address);

        // Limit üstü (101) basım engelleme testi (Atıf işareti temizlendi)
        let over_limit_result = storage.batch_mint_nfts(user_address.clone(), 101);
        assert!(over_limit_result.is_err(), "Hata: Limit ustu (101) basim engellenemedi!");
        
        println!("SUCCESS: All RCP-1 Quest and Native 100-Batch NFT Minting flows passed successfully!");
    }
}