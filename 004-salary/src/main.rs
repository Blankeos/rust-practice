use std::{io::stdout, process::exit, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use num_format::{Buffer, Locale};

fn main() -> std::io::Result<()> {
    // Enable raw mode to capture input directly
    enable_raw_mode()?;

    execute!(
        stdout(),
        cursor::MoveLeft(100),
        Print("🦀"),
        SetForegroundColor(Color::Yellow),
        Print("Welcome"),
        Print(" to the "),
        SetForegroundColor(Color::Reset),
        SetForegroundColor(Color::Blue),
        SetAttribute(Attribute::Underlined),
        Print("Salary Estimator"),
        SetAttribute(Attribute::Reset),
        SetForegroundColor(Color::Reset),
        Print(" App by Carlo. (Written in Rust) \n"),
        cursor::MoveLeft(100),
        Print("\n")
    )?;

    // Values to Select
    let mut selected_frequency = String::new();
    let mut selected_currency = String::new();
    let mut inputed_salary = String::new();

    // ==============================================================================
    // 1. Select "Frequency": Hourly, Daily, Monthly
    // ==============================================================================
    execute!(
        stdout(),
        Print("Select (press [^] [v] to choose):\n"),
        cursor::MoveLeft(100),
        cursor::Hide, // HIDE for now
    )?;

    let frequency_selections = vec!["Hourly", "Daily", "Monthly"];
    let mut frequency_select_idx = 0;
    print_frequency_selections(&frequency_selections, frequency_select_idx, false);

    loop {
        // Poll for events with a timeout
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    exit(0);
                }
                if key_event.code == KeyCode::Down {
                    frequency_select_idx =
                        clamp_array_index(frequency_select_idx + 1, frequency_selections.len() - 1);

                    print_frequency_selections(&frequency_selections, frequency_select_idx, true);
                }
                if key_event.code == KeyCode::Up {
                    frequency_select_idx =
                        clamp_array_index(frequency_select_idx - 1, frequency_selections.len() - 1);

                    print_frequency_selections(&frequency_selections, frequency_select_idx, true);
                }
                if key_event.code == KeyCode::Enter {
                    // It's clamped so let's ignore the error
                    let idx = usize::try_from(frequency_select_idx).unwrap();

                    match frequency_selections.get(idx) {
                        Some(item) => selected_frequency = String::from(*item),
                        None => {}
                    }
                    execute!(stdout(), Print("\n"), cursor::MoveLeft(100))?;
                    break;
                }
            }
        }
    }
    execute!(stdout(), cursor::Show)?;

    // ==============================================================================
    // 2. Select "Currency" and Value
    // ==============================================================================

    let currency_selections = vec!["USD", "PHP"];
    let mut currency_select_idx = 0;

    print_currency_selections(
        &currency_selections,
        currency_select_idx,
        &selected_frequency,
        &selected_currency,
        &inputed_salary,
        false,
    );

    loop {
        // Poll for events with a timeout
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    exit(0);
                }
                if key_event.code == KeyCode::Enter {
                    if inputed_salary.is_empty() {
                        continue;
                    }

                    let idx = usize::try_from(currency_select_idx).unwrap();

                    match currency_selections.get(idx) {
                        Some(item) => selected_currency = String::from(*item),
                        None => {}
                    }

                    break;
                }
                if key_event.code == KeyCode::Left {
                    currency_select_idx =
                        clamp_array_index(currency_select_idx - 1, currency_selections.len() - 1);
                }

                if key_event.code == KeyCode::Right {
                    currency_select_idx =
                        clamp_array_index(currency_select_idx + 1, currency_selections.len() - 1);
                }

                if [
                    KeyCode::Char('1'),
                    KeyCode::Char('2'),
                    KeyCode::Char('3'),
                    KeyCode::Char('4'),
                    KeyCode::Char('5'),
                    KeyCode::Char('6'),
                    KeyCode::Char('7'),
                    KeyCode::Char('8'),
                    KeyCode::Char('9'),
                    KeyCode::Char('0'),
                ]
                .contains(&key_event.code)
                {
                    if let Some(c) = key_event_to_char(&key_event) {
                        inputed_salary.push(c);
                    }
                }

                if KeyCode::Backspace == key_event.code {
                    if !inputed_salary.is_empty() {
                        inputed_salary.pop();
                    }
                }

                print_currency_selections(
                    &currency_selections,
                    currency_select_idx,
                    &selected_frequency,
                    &selected_currency,
                    &inputed_salary,
                    true,
                );
            }
        }
    }

    // ==============================================================================
    // 3. Parse and Calculate Salary
    // ==============================================================================

    let salary = calculate_salary(&inputed_salary, &selected_currency, &selected_frequency);

    let _rounded_salary = salary.round() as i64;

    let mut buf = Buffer::default();
    buf.write_formatted(&_rounded_salary, &Locale::en);

    execute!(
        stdout(),
        Print("\n\n"),
        cursor::MoveLeft(100),
        Print("Since you are earning "),
        Print(selected_currency),
        Print(" "),
        Print(inputed_salary),
        Print(" "),
        Print(selected_frequency),
        Print("\n"),
        cursor::MoveLeft(100),
        Print("Assuming you work 8 hours a day,"),
        Print(" 5 days a week,"),
        Print(" 4 weeks a month,"),
        Print(" and 12 months a year.\n\n"),
        cursor::MoveLeft(100),
        Print("You are expected to make "),
        SetForegroundColor(Color::Green),
        SetAttribute(Attribute::Bold),
        Print("PHP "),
        Print(buf.as_str()),
        SetForegroundColor(Color::Reset),
        SetAttribute(Attribute::Reset),
        Print(" a year!"),
    )?;

    // ==============================================================================
    // 4. Exit Program
    // ==============================================================================

    // Move the cursor to the next line and print the exit message
    execute!(
        stdout(),
        Print("\n\n"),
        cursor::MoveLeft(100),
        Print("🎉 Thanks for using, bye!\n"),
        cursor::MoveLeft(100),
    )?;

    // Disable raw mode before exiting
    disable_raw_mode()?;
    // println!("Exited.");

    Ok(())
}

fn key_event_to_char(event: &KeyEvent) -> Option<char> {
    match event.code {
        KeyCode::Char(c) => Some(c),
        KeyCode::Enter => Some('\n'),
        KeyCode::Tab => Some('\t'),
        // Add more cases for other special characters as needed
        _ => None,
    }
}

/// Prints the "selections" with a current choice emoji
///
/// ### Parameters
/// - `clear_previous`: Only set this to false when you call this func the first time. Default: `true`.
fn print_frequency_selections(
    selections: &Vec<&str>,
    current_selection_idx: i32,
    clear_previous: bool,
) {
    let _current_selection_idx_u32 = usize::try_from(current_selection_idx)
        .expect("[print_currency_selections] current_selection_idx can't be converted to u32.");

    // Clear the previous selection print.
    if clear_previous {
        for (i, _) in selections.iter().enumerate() {
            execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
            execute!(stdout(), cursor::MoveUp(1)).unwrap();

            // If last, also clear.
            if i == selections.len() - 1 {
                execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
            }
        }
    }

    // Print them.
    for (i, selection) in selections.iter().enumerate() {
        if _current_selection_idx_u32 == i {
            execute!(
                stdout(),
                cursor::MoveLeft(100),
                Print("👉\t"),
                SetForegroundColor(Color::Yellow),
                Print(selection),
                SetForegroundColor(Color::Reset),
            )
            .unwrap();
        } else {
            execute!(
                stdout(),
                cursor::MoveLeft(100),
                Print("\t"),
                Print(selection),
            )
            .unwrap();
        }

        execute!(stdout(), Print("\n"), cursor::MoveLeft(100)).unwrap();
    }
}

fn print_currency_selections(
    selections: &Vec<&str>,
    current_selection_idx: i32,
    selected_frequency: &str,
    selected_currency: &str,
    inputed_salary: &str,
    clear_previous: bool,
) {
    execute!(stdout(), cursor::MoveLeft(100)).unwrap();

    // Clear only twice because this particular TUI is only 2 lines.
    if clear_previous {
        for _ in 0..1 {
            execute!(stdout(), Clear(ClearType::CurrentLine), cursor::MoveUp(1)).unwrap();
        }
    }

    let _current_selection_idx_u32 = usize::try_from(current_selection_idx)
        .expect("[print_currency_selections] current_selection_idx can't be converted to u32.");

    // 1. Print [<] USD | PHP [>]
    execute!(stdout(), Print("[<] ")).unwrap();

    for (i, selection) in selections.iter().enumerate() {
        // Display Item
        if i == _current_selection_idx_u32 {
            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                Print(selection),
            )
            .unwrap();
        } else {
            execute!(stdout(), Print(selection)).unwrap();
        }

        execute!(stdout(), SetForegroundColor(Color::Reset)).unwrap();

        // Divider
        if i != selections.len() - 1 {
            execute!(stdout(), Print(" | ")).unwrap();
        }
    }

    execute!(stdout(), Print(" [>]\n"), cursor::MoveLeft(100)).unwrap();

    // 2. Print Currency Input
    execute!(
        stdout(),
        Print("✨ What's your "),
        SetForegroundColor(Color::Yellow),
        Print(selected_frequency),
        SetForegroundColor(Color::Reset),
        Print(" Salary "),
        Print(selected_currency),
        Print(": "),
        Print(inputed_salary)
    )
    .unwrap();
}

/// This func is necessary because len is usize, and if we try to subtract -1 to it when it's 0, we get an error.
/// So I have a custom clamp function to make sure that when I try to modify it
/// - And it becomes less than 0, we just return a clamped 0 of it.
/// - Plus, we also clamp it to the max.
fn clamp_array_index(num: i32, max: usize) -> i32 {
    let _maxi32 = i32::try_from(max).expect("[clamp_array_index] max can't become i32.");

    // The num becomes negative.
    if num < 0 {
        return 0;
    }

    // The num becomes more than max.
    if num >= _maxi32 {
        return _maxi32;
    }

    // The num is valid.
    return num;
}

fn calculate_salary(inputed_salary: &str, currency: &str, frequency: &str) -> f64 {
    const USD_EXCHANGERATE: f64 = 55.75;

    let mut salary: f64 = inputed_salary
        .trim()
        .parse()
        .expect("Couldn't parse inputed_salary");

    if currency == "USD" {
        salary *= USD_EXCHANGERATE;
    }

    // Salary * 5 days a week, 4 weeks a month, 12 months a year.
    if frequency == "Daily" {
        salary = salary * 5.0 * 4.0 * 12.0;
    } else if frequency == "Hourly" {
        salary = salary * 8.0 * 5.0 * 4.0 * 12.0;
    } else if frequency == "Monthly" {
        salary = salary * 12.0;
    }

    return salary;
}
