mod anki_db;

use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use anyhow::Result;
use zip::ZipArchive;

use self::anki_db::AnkiDbExtractor;

use super::card_format::CardAdapter;

/// Read the anki file [file-name].apkg
pub fn read_anki_files<P1: AsRef<Path>, P2: AsRef<Path>>(
    path: P1,
    directory: P2,
) -> Result<(PathBuf, Vec<CardAdapter>)> {
    let mut output_dir = directory.as_ref().to_path_buf();
    output_dir.push(path.as_ref().file_stem().unwrap());

    let media_dir = output_dir.join("media");

    {
        // Extract the anki files
        let apkg_file = fs::File::open(path)?;
        let mut archive = ZipArchive::new(BufReader::new(apkg_file))?;
        archive.extract(&media_dir)?;
    }

    let anki_extractor = AnkiDbExtractor::open(media_dir.join("collection.anki2"))?;

    // Get the media (audio,videos) names.
    let media_data = media_files_store(&media_dir.join("media"))?;

    //Rename with the correct extension
    media_data
        .iter()
        .for_each(|(k, v)| fs::rename(media_dir.join(v), media_dir.join(k)).unwrap());

    // Create the model json file (cards configuration)
    let models_parsed = anki_extractor.get_template_parsed()?;

    let cards_data_raw = anki_extractor.get_cards()?;
    let mut cards_adapters = vec![];

    for card_raw_data in cards_data_raw {
        let model_card = models_parsed.get(&card_raw_data.model_id).unwrap();
        let template_card = &model_card.tmpls[card_raw_data.template_id as usize];

        let mut card_info = CardAdapter::default();

        card_info.front_format = template_card.qfmt.clone();
        card_info.back_format = template_card.afmt.clone();

        model_card.flds.iter().for_each(|fdl| {
            card_info
                .items_format
                .push_str(&format!("{}{}", fdl.name, 0x1f as char));
        });
        //Trim the final ','
        card_info.items_format = card_info.items_format.trim_end_matches(0x1f as char).into();

        for itm in card_raw_data.text.split(0x1f as char) {
            let parsed_text = if itm.contains("sound:") {
                let audio = itm.split(":").last().unwrap().trim_end_matches("]");
                format!("$audio:{}", audio)
            } else if itm.contains("<img") {
                let image = itm.split("\"").nth(1).unwrap();
                format!("$image:{}", image)
            } else {
                itm.to_owned()
            };
            card_info
                .text_items
                .push_str(&format!("{parsed_text}{}", 0x1f as char));
        }

        //Trim the final ','
        card_info.items_format = card_info.items_format.trim_end_matches(0xf1 as char).into();
        cards_adapters.push(card_info);
    }

    let _ = anki_extractor.close();
    let _ = fs::remove_file(media_dir.join("collection.anki2"));
    let _ = fs::remove_file(media_dir.join("media"));

    Ok((output_dir, cards_adapters))
}

fn media_files_store<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, String>> {
    let file_content = fs::read_to_string(path)?;

    let data: HashMap<String, String> = serde_json::from_str(&file_content).unwrap();

    let mut swap_entry = HashMap::new();

    for (file_number, file_name) in data.into_iter() {
        swap_entry.insert(file_name, file_number);
    }

    Ok(swap_entry)
}
