use chrono::Local;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Expense {
    id: u16,
    description: String,
    amount: f32,
    date: String,
}

#[derive(Parser, Debug)]
#[clap(name = "expense_tracker", version = "1.0", author = "Your Name")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(name = "add", about = "Add a new expense")]
    Add { description: String, amount: f32 },
    #[clap(name = "list", about = "List all expenses")]
    List,
    #[clap(name = "summary", about = "Show the total amount of expenses")]
    Summary,
    #[clap(name = "delete", about = "Delete an expense")]
    Delete { id: u16 },
}

impl Expense {
    fn new(description: String, amount: f32, id: u16) -> Expense {
        Expense {
            description,
            amount,
            date: Local::now().format("%Y-%m-%d").to_string(),
            id,
        }
    }
}

fn list_expenses(expenses: &Vec<Expense>) {
    for expense in expenses {
        println!(
            "{}: {} - ${} - {}",
            expense.id, expense.description, expense.amount, expense.date
        );
    }
}

fn get_expenses() -> Vec<Expense> {
    let path = Path::new("expenses.csv");
    if path.exists() {
        let mut reader = csv::Reader::from_path("expenses.csv").expect("Could not read file");
        let mut expenses = Vec::new();
        for result in reader.records() {
            let record = result.expect("Could not read record");
            let id = record[0].parse::<u16>().expect("Could not parse id");
            let description = record[1].to_string();
            let amount = record[2].parse::<f32>().expect("Could not parse amount");
            let date = record[3].to_string();
            expenses.push(Expense {
                id,
                description,
                amount,
                date,
            });
        }
        return expenses;
    }
    Vec::new()
}

fn save_expenses(expenses: &Vec<Expense>) {
    // Save expenses to a file
    let csv = convert_to_csv(expenses);
    let mut file = File::create("expenses.csv").expect("Could not create file");
    file.write_all(csv.as_bytes())
        .expect("Could not write to file");
}

fn convert_to_csv(expenses: &Vec<Expense>) -> String {
    let mut writer = csv::Writer::from_writer(vec![]);

    for expense in expenses {
        writer.serialize(expense).expect("Could not write record");
    }

    let data = writer.into_inner().expect("Could not write record");
    String::from_utf8(data).expect("Could not convert to string")
}

fn add_expense(expenses: &mut Vec<Expense>, description: String, amount: f32) {
    let id = (expenses.len() + 1) as u16;
    let expense = Expense::new(description, amount, id);
    expenses.push(expense);
}

fn summary(expenses: &Vec<Expense>) -> f32 {
    let mut total = 0.0;
    for expense in expenses {
        total += expense.amount;
    }
    total
}

fn delete(expenses: &mut Vec<Expense>, id: u16) {
    let mut index = None;
    for (i, expense) in expenses.iter().enumerate() {
        if expense.id == id {
            index = Some(i);
            break;
        }
    }

    match index {
        Some(i) => {
            expenses.remove(i);
            save_expenses(expenses);
        }
        None => println!("Expense not found"),
    }
}

fn main() {
    let cli = Cli::parse();
    let mut expenses = get_expenses();

    match cli.command {
        Command::Add {
            description,
            amount,
        } => {
            add_expense(&mut expenses, description, amount);
            save_expenses(&expenses);
        }
        Command::List => {
            list_expenses(&expenses);
        }
        Command::Summary => {
            let total = summary(&expenses);
            println!("Total expenses: ${}", total);
        }
        Command::Delete { id } => {
            delete(&mut expenses, id);
        }
    }
}
