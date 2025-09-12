use serde_json::json;

#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
    Text,
    Image,
    Binary,
}

impl ContentType {
    pub fn mime_type(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Text => "text/plain; charset=utf-8",
            ContentType::Image => "image/png",
            ContentType::Binary => "application/octet-stream",
        }
    }
}

pub struct ContentGenerator {
    nft_metadata_templates: Vec<serde_json::Value>,
    text_templates: Vec<String>,
}

impl ContentGenerator {
    pub fn new() -> Self {
        let nft_metadata_templates = vec![
            json!({
                "name": "Epic Battle Axe",
                "description": "A legendary weapon forged in the depths of Mount Bunker. This axe has seen countless battles and carries the spirit of ancient warriors.",
                "image": "ipfs://QmExampleImageHash123456789",
                "attributes": [
                    {
                        "trait_type": "Weapon Type",
                        "value": "Axe"
                    },
                    {
                        "trait_type": "Rarity",
                        "value": "Epic"
                    },
                    {
                        "trait_type": "Damage",
                        "value": 150
                    },
                    {
                        "trait_type": "Durability",
                        "value": 85
                    },
                    {
                        "trait_type": "Element",
                        "value": "Fire"
                    }
                ],
                "external_url": "https://bunkerverse.io/items/epic-battle-axe",
                "animation_url": "ipfs://QmExampleAnimationHash987654321"
            }),
            json!({
                "name": "Stealth Armor Set",
                "description": "Advanced nano-fiber armor that provides enhanced stealth capabilities and protection against both physical and energy-based attacks.",
                "image": "ipfs://QmArmorImageHash456789123",
                "attributes": [
                    {
                        "trait_type": "Armor Type",
                        "value": "Full Set"
                    },
                    {
                        "trait_type": "Rarity",
                        "value": "Legendary"
                    },
                    {
                        "trait_type": "Defense",
                        "value": 200
                    },
                    {
                        "trait_type": "Stealth Bonus",
                        "value": "+25%"
                    },
                    {
                        "trait_type": "Energy Shield",
                        "value": 75
                    }
                ],
                "external_url": "https://bunkerverse.io/items/stealth-armor"
            }),
            json!({
                "name": "Quantum Generator",
                "description": "A portable quantum energy generator that can power entire settlements. Essential for establishing bases in remote locations.",
                "image": "ipfs://QmGeneratorImageHash789123456",
                "attributes": [
                    {
                        "trait_type": "Item Type",
                        "value": "Utility"
                    },
                    {
                        "trait_type": "Rarity",
                        "value": "Ultra Rare"
                    },
                    {
                        "trait_type": "Power Output",
                        "value": "50 MW"
                    },
                    {
                        "trait_type": "Efficiency",
                        "value": "99.2%"
                    },
                    {
                        "trait_type": "Fuel Type",
                        "value": "Quantum Cells"
                    }
                ],
                "external_url": "https://bunkerverse.io/items/quantum-generator"
            }),
            json!({
                "name": "Digital Avatar Fragment",
                "description": "A rare fragment containing the consciousness pattern of an ancient AI entity. Can be used to enhance player abilities or unlock hidden knowledge.",
                "image": "ipfs://QmAvatarFragmentHash321654987",
                "attributes": [
                    {
                        "trait_type": "Fragment Type",
                        "value": "Consciousness"
                    },
                    {
                        "trait_type": "Rarity",
                        "value": "Mythic"
                    },
                    {
                        "trait_type": "AI Level",
                        "value": "Tier IX"
                    },
                    {
                        "trait_type": "Knowledge Domain",
                        "value": "Quantum Physics"
                    },
                    {
                        "trait_type": "Compatibility",
                        "value": "Universal"
                    }
                ],
                "external_url": "https://bunkerverse.io/items/avatar-fragment"
            }),
        ];

        let text_templates = vec![
            "Welcome to the Bunkerverse Platform - Your gateway to decentralized gaming experiences.".to_string(),
            "This is a sample text file stored on IPFS. The content can be accessed via the IPFS gateway and represents immutable data storage.".to_string(),
            "Bunkerverse Mission Briefing:\n\nObjective: Secure the quantum reactor\nLocation: Sector 7-Alpha\nRisk Level: High\n\nIntel suggests enemy forces have established a perimeter around the facility. Stealth approach recommended.".to_string(),
            "Player Achievement Unlocked!\n\nYou have successfully completed the \"First Steps\" mission series.\n\nRewards:\n- 500 Experience Points\n- Epic Battle Axe\n- Access to Advanced Training Facility".to_string(),
            "===== Bunkerverse Platform Status Report =====\n\nNetwork Status: ONLINE\nActive Players: 12,543\nActive Missions: 89\nMarketplace Listings: 2,341\n\nSystem Status: All services operational".to_string(),
        ];

        Self {
            nft_metadata_templates,
            text_templates,
        }
    }

    pub fn generate_content(&self, cid: &str) -> (Vec<u8>, String) {
        let content_type = self.determine_content_type(cid);

        match content_type {
            ContentType::Json => {
                let template = self.select_nft_template(cid);
                let json_content =
                    serde_json::to_vec_pretty(&template).unwrap_or_else(|_| b"{}".to_vec());
                (json_content, content_type.mime_type().to_string())
            }
            ContentType::Text => {
                let text_content = self.select_text_template(cid);
                (
                    text_content.into_bytes(),
                    content_type.mime_type().to_string(),
                )
            }
            ContentType::Image => {
                let image_content = self.generate_mock_image(cid);
                (image_content, content_type.mime_type().to_string())
            }
            ContentType::Binary => {
                let binary_content = self.generate_mock_binary(cid);
                (binary_content, content_type.mime_type().to_string())
            }
        }
    }

    fn determine_content_type(&self, cid: &str) -> ContentType {
        // Determine content type based on CID patterns
        let cid_lower = cid.to_lowercase();

        if cid_lower.contains("json")
            || cid_lower.contains("meta")
            || cid_lower.ends_with("1")
            || cid_lower.ends_with("3")
        {
            ContentType::Json
        } else if cid_lower.contains("image")
            || cid_lower.contains("png")
            || cid_lower.contains("jpg")
            || cid_lower.ends_with("2")
            || cid_lower.ends_with("6")
        {
            ContentType::Image
        } else if cid_lower.contains("txt")
            || cid_lower.contains("text")
            || cid_lower.ends_with("4")
            || cid_lower.ends_with("8")
        {
            ContentType::Text
        } else {
            // Default to JSON for NFT metadata if no specific pattern matches
            ContentType::Json
        }
    }

    fn select_nft_template(&self, cid: &str) -> serde_json::Value {
        let index = self.hash_string_to_index(cid, self.nft_metadata_templates.len());
        let mut template = self.nft_metadata_templates[index].clone();

        // Customize the template based on the CID
        if let Some(obj) = template.as_object_mut() {
            // Update image and animation URLs to use the requesting CID or related ones
            obj.insert(
                "tokenId".to_string(),
                json!(self.hash_string_to_index(cid, 9999) + 1),
            );
            obj.insert(
                "edition".to_string(),
                json!(self.hash_string_to_index(cid, 100) + 1),
            );
            obj.insert(
                "generatedAt".to_string(),
                json!(chrono::Utc::now().to_rfc3339()),
            );
            obj.insert("ipfsHash".to_string(), json!(cid));

            // Add some randomization to attributes based on CID
            if let Some(attributes) = obj.get_mut("attributes").and_then(|v| v.as_array_mut()) {
                for attr in attributes {
                    if let Some(attr_obj) = attr.as_object_mut() {
                        if let Some(trait_type) =
                            attr_obj.get("trait_type").and_then(|v| v.as_str())
                        {
                            match trait_type {
                                "Damage" | "Defense" | "Power Output" => {
                                    if let Some(val) =
                                        attr_obj.get("value").and_then(|v| v.as_i64())
                                    {
                                        let variance =
                                            (self.hash_string_to_index(cid, 20) as i64) - 10;
                                        attr_obj.insert("value".to_string(), json!(val + variance));
                                    }
                                }
                                "Durability" | "Efficiency" => {
                                    let base_val = 50 + (self.hash_string_to_index(cid, 50) as i64);
                                    attr_obj.insert("value".to_string(), json!(base_val));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        template
    }

    fn select_text_template(&self, cid: &str) -> String {
        let index = self.hash_string_to_index(cid, self.text_templates.len());
        let mut template = self.text_templates[index].clone();

        // Add CID-specific customization
        template.push_str(&format!(
            "\n\nContent ID: {}\nGenerated at: {}",
            cid,
            chrono::Utc::now().to_rfc3339()
        ));

        template
    }

    fn generate_mock_image(&self, cid: &str) -> Vec<u8> {
        // Generate a simple PNG image based on CID
        // This is a minimal 1x1 pixel PNG with color derived from CID hash
        let color_seed = self.hash_string_to_index(cid, 255) as u8;

        // PNG signature + IHDR + pixel data + IEND
        let mut png_data = Vec::new();

        // PNG signature
        png_data.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);

        // IHDR chunk (13 bytes)
        png_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D]); // Length
        png_data.extend_from_slice(b"IHDR"); // Type
        png_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x64]); // Width: 100
        png_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x64]); // Height: 100
        png_data.extend_from_slice(&[0x08, 0x02, 0x00, 0x00, 0x00]); // Bit depth, color type, etc.
        png_data.extend_from_slice(&[0x7D, 0xD8, 0x4B, 0x8D]); // CRC

        // Simple IDAT chunk with solid color based on CID
        let idat_data = vec![
            0x78, 0x9C, 0xED, 0xC1, 0x01, 0x01, 0x00, 0x00, 0x00, 0x80, 0x90, 0xFE, 0x37, 0x02,
            color_seed, 0x01,
        ];
        png_data.extend_from_slice(&(idat_data.len() as u32).to_be_bytes());
        png_data.extend_from_slice(b"IDAT");
        png_data.extend_from_slice(&idat_data);
        png_data.extend_from_slice(&[0x9D, 0x3E, 0x53, 0x18]); // CRC

        // IEND chunk
        png_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Length
        png_data.extend_from_slice(b"IEND"); // Type
        png_data.extend_from_slice(&[0xAE, 0x42, 0x60, 0x82]); // CRC

        png_data
    }

    fn generate_mock_binary(&self, cid: &str) -> Vec<u8> {
        // Generate binary data based on CID hash
        let mut data = Vec::new();
        let seed = self.hash_string_to_index(cid, 1000);

        // Create deterministic binary data
        for i in 0..1024 {
            data.push(((seed + i) % 256) as u8);
        }

        data
    }

    fn hash_string_to_index(&self, input: &str, modulo: usize) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        (hasher.finish() as usize) % modulo
    }
}

impl Default for ContentGenerator {
    fn default() -> Self {
        Self::new()
    }
}
