fn main() {
    println!("Day 2 Part 1: {}", run_input_for_day_2(12, 2));
    for i in (0..99) {
        for j in (0..99) {
            let value = run_input_for_day_2(i, j);
            if value == 19690720i32 {
                println!("Day 2 Part 2: {}", 100 * i + j);
            }
        }
    }
}

/**

 --- Day 2: 1202 Program Alarm ---

 On the way to your gravity assist around the Moon, your ship computer beeps angrily about a "1202 program alarm". On the radio, an Elf is already explaining how to handle the situation: "Don't worry, that's perfectly norma--" The ship computer bursts into flames.

 You notify the Elves that the computer's magic smoke seems to have escaped. "That computer ran Intcode programs like the gravity assist program it was working on; surely there are enough spare parts up there to build a new Intcode computer!"

 An Intcode program is a list of integers separated by commas (like 1,0,0,3,99). To run one, start by looking at the first integer (called position 0). Here, you will find an opcode - either 1, 2, or 99. The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt. Encountering an unknown opcode means something went wrong.

 Opcode 1 adds together numbers read from two positions and stores the result in a third position. The three integers immediately after the opcode tell you these three positions - the first two indicate the positions from which you should read the input values, and the third indicates the position at which the output should be stored.

 For example, if your Intcode computer encounters 1,10,20,30, it should read the values at positions 10 and 20, add those values, and then overwrite the value at position 30 with their sum.

 Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them. Again, the three integers after the opcode indicate where the inputs and outputs are, not their values.

 Once you're done processing an opcode, move to the next one by stepping forward 4 positions.

 For example, suppose you have the following program:

 1,9,10,3,2,3,11,0,99,30,40,50
 For the purposes of illustration, here is the same program split into multiple lines:

 1,9,10,3,
 2,3,11,0,
 99,
 30,40,50
 The first four integers, 1,9,10,3, are at positions 0, 1, 2, and 3. Together, they represent the first opcode (1, addition), the positions of the two inputs (9 and 10), and the position of the output (3). To handle this opcode, you first need to get the values at the input positions: position 9 contains 30, and position 10 contains 40. Add these numbers together to get 70. Then, store this value at the output position; here, the output position (3) is at position 3, so it overwrites itself. Afterward, the program looks like this:

 1,9,10,70,
 2,3,11,0,
 99,
 30,40,50
 Step forward 4 positions to reach the next opcode, 2. This opcode works just like the previous, but it multiplies instead of adding. The inputs are at positions 3 and 11; these positions contain 70 and 50 respectively. Multiplying these produces 3500; this is stored at position 0:

 3500,9,10,70,
 2,3,11,0,
 99,
 30,40,50
 Stepping forward 4 more positions arrives at opcode 99, halting the program.

 Here are the initial and final states of a few more small programs:

 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
 Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?
**/
fn get_input_for_day_2() -> Vec<i32> {
    vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 5, 19, 23, 2, 9, 23, 27,
        1, 27, 5, 31, 2, 31, 13, 35, 1, 35, 9, 39, 1, 39, 10, 43, 2, 43, 9, 47, 1, 47, 5, 51, 2,
        13, 51, 55, 1, 9, 55, 59, 1, 5, 59, 63, 2, 6, 63, 67, 1, 5, 67, 71, 1, 6, 71, 75, 2, 9, 75,
        79, 1, 79, 13, 83, 1, 83, 13, 87, 1, 87, 5, 91, 1, 6, 91, 95, 2, 95, 13, 99, 2, 13, 99,
        103, 1, 5, 103, 107, 1, 107, 10, 111, 1, 111, 13, 115, 1, 10, 115, 119, 1, 9, 119, 123, 2,
        6, 123, 127, 1, 5, 127, 131, 2, 6, 131, 135, 1, 135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0, 0,
    ]
}

fn run_input_for_day_2(input_1: i32, input_2: i32) -> i32 {
    let mut input_vec = get_input_for_day_2();
    input_vec[1] = input_1;
    input_vec[2] = input_2;

    for index in (0..input_vec.len()).step_by(4) {
        //for (index, code) in input_vec.iter_mut().step_by(4).enumerate() {
        let code = input_vec[index];

        if code == 99i32 {
            break;
        }
        process_op_code(code, index, &mut input_vec);
    }

    //println!("Vec status: {:?}", input_vec);
    return input_vec[0];
}

fn process_op_code(code: i32, current_index: usize, _borrowed_input_vec: &mut Vec<i32>) {
    let pos1 = _borrowed_input_vec[current_index + 1] as usize;
    let pos2 = _borrowed_input_vec[current_index + 2] as usize;
    let output_pos = _borrowed_input_vec[current_index + 3] as usize;
    //println!("Code: {}, Pos1: {}, Pos2: {}, OutputPos: {}", code, pos1, pos2, output_pos);

    let value_1 = _borrowed_input_vec[pos1];
    let value_2 = _borrowed_input_vec[pos2];

    let value = if code == 1i32 {
        value_1 + value_2
    } else {
        value_1 * value_2
    };

    //println!("Value1: {}, Value2: {}, Value: {}", value_1, value_2, value);
    _borrowed_input_vec[output_pos] = value;
}
