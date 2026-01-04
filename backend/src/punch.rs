use common::*;
const STANDARD: &str = "abcdefghijklmnopqrstuvwxyz";


pub async fn decrypt(my_rotors: Vec<CipherRotor>,
    reflector: Reflector, message: String,
    start_position: Vec<String>, plugboard: String) -> (String, Vec<DebugLogs>) {

        let mut offset_array: Vec<i32> = [
            STANDARD.find(&start_position[0].to_lowercase()).unwrap_or_else(|| 0) as i32,
            STANDARD.find(&start_position[1].to_lowercase()).unwrap_or_else(|| 0) as i32,
            STANDARD.find(&start_position[2].to_lowercase()).unwrap_or_else(|| 0) as i32].to_vec();
        let mut cpt_letters = 0;
        let mut debug_logs_list: Vec<DebugLogs> = vec![];
        let mut message_vec: Vec<char> = message.chars().collect();
        let mut ticked = [false, false, false];

        for (loop_char, current_letter) in message_vec.clone().iter().enumerate() {
            // Ignore punctuations, special characters & non standard alphabet
            if current_letter.is_alphabetic() &&
                STANDARD.find(*current_letter) != None {

                let mut my_logs: Vec<String> = vec![];

                // Ticking fast rotor
                (offset_array[0], ticked[0]) = tick_rotor(vec![], offset_array[0], 1);

                let mut result_letter = *current_letter;
                let mut deb_letter = result_letter;

                // Check if we have to swap the letter with the plugboard
                result_letter = plugboard_swich(result_letter, plugboard.clone());
                my_logs.push(format!("[{}] - Plugboard ", deb_letter));

                // WIRING. Forward Path Right > Left
                for rot in 0..3 {
                    result_letter = wire(result_letter, my_rotors[rot].definition.to_lowercase().clone(), offset_array[rot]);
                    my_logs.push(format!("[{}] ↣ Rotor: {}", deb_letter, my_rotors[rot].name));
                    deb_letter = result_letter;
                 }

                // Matching with Reflector
                result_letter = reflector.definition.to_lowercase().chars().nth(STANDARD.find(result_letter).unwrap()).unwrap();
                my_logs.push(format!("[{}] ⟲ {}", result_letter, reflector.name));

                // REVERSE. Forward Path Right > Left
                for rot in (0..3).rev() {
                    result_letter = reverse(result_letter, my_rotors[rot].definition.to_lowercase().clone(), offset_array[rot]);

                    my_logs.push(format!("Rotor: {} ↢ [{}]",my_rotors[rot].name, result_letter));

                    //deb_letter = result_letter;
               }

                // PLUGBOARD. Check if we have one defined
                deb_letter = result_letter;
                result_letter = plugboard_swich(result_letter, plugboard.clone());
                my_logs.push(format!("[{}] - Plugboard - [{}]", deb_letter, result_letter));


               // replacing the character back into the vector
               message_vec[loop_char] = result_letter;

               // For logs and debugging only
                let mut offset_char: Vec<char> = vec![];
                for el in offset_array.iter() {
                    offset_char.push(STANDARD.chars().nth((*el) as usize).unwrap_or_else(|| '-'));
                }

                debug_logs_list.push(
                    common::DebugLogs { idx: cpt_letters, offset: offset_char.clone(), pass: my_logs}
                );


                if ticked[1] {
                    (offset_array[2], ticked[2]) = tick_rotor(my_rotors[1].notch.clone(), offset_array[2], offset_array[1]);
                }

                // Check medium & slow rotors only if we already ticked with the previous one
                // must tick middle rotor if the slow motor ticked
                if ticked[0] {
                    (offset_array[1], ticked[1]) = tick_rotor(my_rotors[0].notch.clone(), offset_array[1], offset_array[0]);
                    if ticked[2] {
                        (offset_array[1], ticked[1]) = tick_rotor(vec![], offset_array[1], offset_array[0]);
                    };
                };

                cpt_letters += 1;
            }
        }
        // Converting working vector to string
        (message_vec.iter().cloned().collect::<String>(), debug_logs_list)
}
fn move_next_through_set(set: String, current_index: i32, offset: i32) -> char {
    let set_len: i32 = set.len().try_into().unwrap();
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
    let result_letter = move_next_through_set(
        rotor.clone(),
        character.to_ascii_lowercase() as i32 - 97,
        offset);
    // Getting matching character from standard alphabet with Offset back
    move_prev_through_set(
        STANDARD.to_string(),
        result_letter.to_ascii_lowercase() as i32 - 97,
        offset)
}
fn reverse(character: char, rotor: String, offset: i32) -> char {
    // Getting matching character from the ROTOR with Offset
    let result_letter = move_next_through_set(
        STANDARD.to_string(),
        STANDARD.find(character).unwrap() as i32,
        offset);

    // Getting matching character from standard alphabet with Offset back
    move_prev_through_set(
        STANDARD.to_string(),
        rotor.find(result_letter).unwrap() as i32,
        offset)
}

fn tick_rotor(notches: Vec<char>, mut current_offset: i32, ticking_notch: i32) -> (i32, bool) {
    // exception for the first rotor ticking each round
    let mut ticked = false;
    if notches.len() == 0 {
       ticked = true;
       if current_offset == 25 {
           current_offset = 0;
       } else {
           current_offset += 1; }
    }

    for n in notches {
        if (n.to_ascii_lowercase() as i32 - 97) == ticking_notch {
            if current_offset == 25 {
                current_offset = 0;
                ticked = true;
            } else {
              current_offset += 1;
              ticked = true;
            }
        }
    }
    (current_offset, ticked)
}
fn plugboard_swich(c: char, plugboard: String) -> char {
    let mut res: char = c;
    for w in plugboard.split_whitespace() {
        let pos = w.find(c);

        if pos != None {
            if pos == Some(1) { res = w.as_bytes()[0] as char; }
            else { res = w.as_bytes()[1] as char; }
        }
    }
    res
}
