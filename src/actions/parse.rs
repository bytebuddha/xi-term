use clap::{ clap_app, AppSettings, ArgMatches, Error as ClapError };

use super::*;
use components::{ PromptResponse, Message };

const HELP_TEMPLATE: &'static str ="USAGE:

    {usage}

FLAGS
{flags}

SUBCOMMANDS
{subcommands}";

fn get_matches(input: Vec<&str>) -> Result<ArgMatches<'static>, ClapError> {
    clap_app!(app =>
            (usage: "<COMMAND> <ARGS>")
            (setting: AppSettings::SubcommandsNegateReqs)
            (setting: AppSettings::NoBinaryName)
            (template: HELP_TEMPLATE)
            (global_setting: AppSettings::ColorNever)
            (@arg cmd: +raw +takes_value )
            (@subcommand editor =>
                (about: "Manage the editor")
                (setting: AppSettings::SubcommandRequiredElseHelp)
                (template: HELP_TEMPLATE)
                (@subcommand view =>
                    (about: "Mange editor views")
                    (setting: AppSettings::SubcommandRequiredElseHelp)
                    (template: HELP_TEMPLATE)
                    (@subcommand save =>
                         (about: "Save the current buffer")
                         (template: HELP_TEMPLATE)
                         (@arg file: -f --file +takes_value "The file name to save the current view as")
                    )
                    (@subcommand next =>
                        (about: "Move the the next view")
                    )
                    (@subcommand prev =>
                        (about: "Move to the previous buffer")
                    )
                    (@subcommand lang =>
                        (setting: AppSettings::SubcommandRequiredElseHelp)
                        (@subcommand list =>
                            (about: "List all available languages")
                        )
                        (@subcommand set =>
                            (about: "Set the language of the current view")
                            (@arg lang: -l --lang +required +takes_value "The name of the language to set")
                        )
                    )
                    (@subcommand cursor =>
                        (setting: AppSettings::SubcommandRequiredElseHelp)
                        (template: HELP_TEMPLATE)
                         (@subcommand up =>)
                         (@subcommand down =>)
                         (@subcommand left =>)
                         (@subcommand right =>)
                         (@subcommand pageup =>)
                         (@subcommand pagedown =>)
                         (@subcommand home =>)
                         (@subcommand end =>)
                         (@subcommand back =>)
                         (@subcommand delete =>)
                    )
                    (@subcommand find =>
                        (@arg query: +takes_value +required "The phrase to search for")
                        (@arg next: -n --next "Find the next Occurance")
                        (@arg prev: -p --prev "Find the previous Occurance")
                    )
                )
                (@subcommand theme =>
                    (about: "Manage syntax themes")
                    (setting: AppSettings::SubcommandRequiredElseHelp)
                    (@subcommand list =>
                         (about: "List all available syntax themes")
                    )
                    (@subcommand set =>
                        (about: "Set the syntax theme in use")
                        (@arg theme: -t --theme +required +takes_value "The name of the theme to set")
                    )
                )
                (@subcommand open =>
                    (about: "Open a new file")
                    (@arg file_name: -f --file +takes_value "The flie name to open")
                )
            )
            (@subcommand settings =>
                (about: "Modify the editor settings")
                (template: HELP_TEMPLATE)
                (setting: AppSettings::SubcommandRequiredElseHelp)
                (@subcommand title =>
                    (about: "Toggle displaying the title bar")
                )
                (@subcommand lines =>
                    (about: "Toggle displaying the syntax gutter")
                )
                (@subcommand debug =>
                    (about: "Toggle the Developer Tools")
                )
            )
    ).get_matches_from_safe(input)
}

pub fn parse_action(input: &str) -> PromptResponse {
    let q = input.split(" ").collect::<Vec<&str>>();
    match get_matches(q) {
        Ok(matches) => {
            parse_matches(matches)
        },
        Err(err) => {
            PromptResponse::Message(Message::error(err.message))
        }
    }
}

fn parse_matches<'a>(matches: ArgMatches<'a>) -> PromptResponse {
    if let Some(command) = matches.values_of("cmd") {
        return PromptResponse::Action(Action::ShellCommand(command.map(|item|item.to_string()).collect()));
    }
    match matches.subcommand() {
        ("editor", Some(matches)) => parse_editor_matches(matches),
        ("settings", Some(matches)) => parse_settings_matches(matches),
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown Editor Command: '{}'", cmd)))
    }
}

fn parse_settings_matches<'a>(matches: &ArgMatches<'a>) -> PromptResponse {
    match matches.subcommand() {
        ("title", _) => PromptResponse::Action(Action::Ui(UiAction::ToggleTitleBar)),
        ("lines", _) => PromptResponse::Action(Action::Ui(UiAction::ToggleLineNumbers)),
        ("debug", _) => PromptResponse::Action(Action::Ui(UiAction::ShowDebugWidget)),
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown Settings Command: '{}'", cmd)))
    }
}

fn parse_editor_matches<'a>(matches: &ArgMatches<'a>) -> PromptResponse {
    match matches.subcommand() {
        ("view", Some(matches)) => parse_view_matches(matches),
        ("theme", Some(matches)) => parse_theme_matches(matches),
        ("open", Some(matches)) => PromptResponse::Action(Action::Editor(EditorAction::Open(matches.value_of("file_name").map(|item|item.to_string())))),
        ("open", None) => PromptResponse::Action(Action::Editor(EditorAction::Open(None))),
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown Editor Command: '{}'", cmd)))
    }
}

fn parse_view_matches<'a>(matches: &ArgMatches<'a>) -> PromptResponse {
    match matches.subcommand() {
        ("cursor", Some(matches)) => parse_cursor_matches(matches),
        ("lang", Some(matches)) => parse_lang_matches(matches),
        ("save", Some(matches)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Save(matches.value_of("file").unwrap().to_string())))),
        ("next", _) => PromptResponse::Action(Action::Editor(EditorAction::NextView)),
        ("prev", _) => PromptResponse::Action(Action::Editor(EditorAction::PrevView)),
        ("find", Some(matches)) => {
            let find = if matches.is_present("next") {
                FindAction::Next(matches.is_present("wrap"), matches.is_present("same"))
            } else if matches.is_present("prev") {
                FindAction::Previous(matches.is_present("wrap"), matches.is_present("same"))
            } else {
                FindAction::Query(
                    matches.value_of("query").unwrap().to_string(),
                    matches.is_present("regex"), matches.is_present("case"),
                    matches.is_present("words")
                )
            };
            PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Find(find))))
        },
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown View Command: '{}'", cmd)))
    }
}

fn parse_theme_matches<'a>(matches: &ArgMatches<'a>) -> PromptResponse {
    match matches.subcommand() {
        ("list", _) => PromptResponse::Action(Action::Editor(EditorAction::ListThemes)),
        ("set", Some(matches)) => PromptResponse::Action(Action::Editor(EditorAction::SetTheme(matches.value_of("theme").unwrap().to_owned()))),
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown Theme Command: '{}'", cmd)))
    }
}

fn parse_lang_matches<'a>(matches: &ArgMatches<'a>) -> PromptResponse {
    match matches.subcommand() {
        ("list", _) => PromptResponse::Action(Action::Editor(EditorAction::ListLanguages)),
        ("set", Some(matches)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::SetLanguage(matches.value_of("lang").unwrap().to_owned())))),
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown Language Command: '{}'", cmd)))
    }
}

fn parse_cursor_matches<'a>(matches: &ArgMatches<'a>) -> PromptResponse {
    match matches.subcommand() {
        ("up", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Up)))),
        ("down", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Down)))),
        ("left", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Left)))),
        ("right", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Right)))),
        ("pageup", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::PageUp)))),
        ("pagedown", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::PageDown)))),
        ("home", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Home)))),
        ("end", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::End)))),
        ("back", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Backspace)))),
        ("delete", Some(_)) => PromptResponse::Action(Action::Editor(EditorAction::View(ViewAction::Cursor(CursorAction::Delete)))),
        (cmd, _) => PromptResponse::Message(Message::error(format!("Unknown Cursor Command: '{}'", cmd)))
    }
}
