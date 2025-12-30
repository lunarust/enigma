use common::*;
const STANDARD: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub async fn decrypt(my_rotors: Vec<CipherRotor>,
    reflector: Reflector, message: String) -> (String, Vec<DebugLogs>) {
        let mut message_upper_case = message.to_uppercase();

        let mut offset_array: Vec<i32> = [0,0,0].to_vec();
        let mut cpt_letters = 0;

        let mut debug_logs_list: Vec<DebugLogs> = vec![];

        for loop_char in 0..message_upper_case.chars().count() {
            let current_letter = message.chars().nth(loop_char).unwrap();
            if current_letter.is_alphabetic() {                                                             // We will ignore punctuations & special characters

                let mut my_logs: Vec<String> = vec![];
                // We need to start ticking the rotors
                if offset_array[0] == 26 { offset_array[0] = 1; } else { offset_array[0] += 1; }
                if (cpt_letters) % 26 == 0 && cpt_letters != 0 {
                   if offset_array[1] == 26 { offset_array[1] = 0; } else { offset_array[1] += 1; }
               }
                if (cpt_letters+1) % 676 == 0 {
                   if offset_array[2] == 26 { offset_array[2] = 0; } else { offset_array[2] += 1; }
               }

                //println!("POSITION {:?}", offset_array );
                //let mut index_position: i32 = (current_letter.to_ascii_lowercase() as i32) - 97;
                let mut result_letter = current_letter;
                let mut deb_letter = current_letter;

                // Forward Path Right > Left
                for rot in 0..3 {
                    result_letter = wire(result_letter, my_rotors[rot].definition.clone(), offset_array[rot]);
                    //println!("[{}] ↣ {} ↣ {}", deb_letter, my_rotors[rot].name, result_letter);
                    my_logs.push(format!("[{}] ↣ {} ↣ {}", deb_letter, my_rotors[rot].name, result_letter));
                    deb_letter = result_letter;
                 }

                // Matching with Reflector
                result_letter = reflector.definition.chars().nth(STANDARD.find(result_letter).unwrap()).unwrap();
                //println!("[{}] ⟲ {} ⟲ {}", deb_letter, reflector.name, result_letter);
                my_logs.push(format!("[{}] ⟲ {} ⟲ {}", deb_letter, reflector.name, result_letter));

                // Forward Path Right > Left
                for rot in (0..3).rev() {
                    result_letter = reverse(result_letter, my_rotors[rot].definition.clone(), offset_array[rot]);
                    //println!("[{}] ↢ {} ↢ {}", deb_letter, my_rotors[rot].name, result_letter);
                    my_logs.push(format!("[{}] ↢ {} ↢ {}", deb_letter, my_rotors[rot].name, result_letter));

                    deb_letter = result_letter;
               }

                message_upper_case.replace_range(
                    (loop_char)..(loop_char+1),
                    result_letter.to_string().as_str()
                );

                debug_logs_list.push(
                    common::DebugLogs {
                    idx: cpt_letters,
                    offset: offset_array.clone(),
                    pass: my_logs}
                );

                cpt_letters += 1;
            }
        }
        //println!("{:?}", debug_logs_list);
        (message_upper_case, debug_logs_list)
}
fn move_next_through_set(set: String, current_index: i32, offset: i32) -> char {
    let set_len: i32 = set.len().try_into().unwrap();
    //println!("set: {}, current_index: {}, offset: {}", set, current_index, offset);
    set.chars().nth(
        ((current_index + offset) % set_len) as usize
    ).unwrap()
}
fn move_prev_through_set(set: String, current_index: i32, offset: i32) -> char {
    let set_len: i32 = set.len().try_into().unwrap();
    set.chars().nth(
        ((current_index - offset + set_len) % set_len) as usize
    ).unwrap()
}
fn wire(character: char, rotor: String, offset: i32) -> char {
    // Getting matching character from the ROTOR with Offset
    let result_letter = move_next_through_set(rotor.clone(), (character.to_ascii_lowercase() as i32) - 97, offset);
    // Getting matching character from standard alphabet with Offset back
    move_prev_through_set(STANDARD.to_string(), (result_letter.to_ascii_lowercase() as i32) - 97, offset)
}
fn reverse(character: char, rotor: String, offset: i32) -> char {
    let result_letter = move_next_through_set(STANDARD.to_string(), STANDARD.find(character).unwrap() as i32, offset);
    move_prev_through_set(STANDARD.to_string(), rotor.find(result_letter).unwrap() as i32, offset)
}
