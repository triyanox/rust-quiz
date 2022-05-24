use std::io::stdin;


struct Question {
    question: String,
    answer: String,
}

fn main () {
    let questions = [
        Question {
            question: "What is not a valid CSS property?\n chose the correct answer \n 1. font-size \n 2. font-color \n 3. font-weight \n 4. font-family".to_string(),
            answer: "2".to_string(),
        },
        Question{
            question : "What is not a javascript library? \n 1. jQuery \n 2. React \n 3. Express \n 4. Yew".to_string(),
            answer : "4".to_string(),
        },
        Question{
            question : "What is not a valid Javascript variable? \n 1. var \n 2. let \n 3. const ".to_string(),
            answer : "3".to_string(),
        },
        Question{
            question : "What is not a valid Javascript operator? \n 1. + \n 2. - \n 3. * \n 4. =".to_string(),
            answer : "4".to_string(),
        },
        Question{
            question : "How to write an array in Javascript? \n 1. var colors = ['red', 'green', 'blue'] \n 2. var colors = (red, green, blue) \n 3. var colors = {red, green, blue} \n 4. var colors = {red: 'red', green: 'green', blue: 'blue'}".to_string(),
            answer : "1".to_string(),
        },
        Question{
            question : "How to remove the last element from an array in Javascript? \n 1. colors.shift() \n 2. colors.pop() \n 3. colors.unshift() \n 4. colors.push()".to_string(),
            answer : "2".to_string(),
        },
        Question {
            question: "What is not a valid Javascript data type? \n 1. string \n 2. number \n 3. boolean \n 4. class".to_string(),
            answer: "4".to_string(),
        },
        Question{
            question : "How to print 'Hello World' in Javascript? \n 1. console.log('Hello World') \n 2. print('Hello World') \n 3. document.write('Hello World') \n 4. alert('Hello World')".to_string(),
            answer : "1".to_string(),
        },
        Question{
            question : "How to add a comment in Javascript? \n 1. // This is a comment \n 2. ** This is a comment ** \n 3. # This is a comment \n 4. % This is a comment".to_string(),
            answer : "1".to_string(),
        },


    ];

    

    let mut score = 0;

    for question in questions.iter() {
        println!("{}", question.question);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == question.answer {
            score += 1;
        }
    }

    println!("You scored {} out of {}", score, questions.len());

}
