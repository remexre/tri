//! Functions to display tasks in pretty tables.
//!
//! TODO: See if there's a lens-like abstraction that could be used to make
//! this less necessary. (Or at least, to have a single highly generic
//! function rather than all the ones that exist).

use std::collections::HashMap;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;
use rayon::prelude::*;

use controller::Tri;
use models::Task;

/// Displays the task ID, assignee, name, priority, status, and due date in a
/// table. Requires access to the Tri controller to be able to lookup names.
pub fn render_all_tasks(tasks: Vec<Task>, tri: &Tri) -> String {
    let users = tasks.iter().map(|task| task.user_id).collect::<Vec<_>>();
    let users = users
        .into_par_iter()
        .filter_map(|id| tri.name_for_id(id, true).map(|name| (id, name)).ok())
        .collect::<HashMap<_, _>>();

    let mut table = Table::new();
    table.set_titles(Row::from(
        &["ID", "Assignee", "Name", "Priority", "Done?", "Due Date"],
    ));
    for task in tasks {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&task.id.to_string()));
        row.add_cell(Cell::new(if let Some(name) = users.get(&task.user_id) {
            name
        } else {
            ""
        }));
        row.add_cell(Cell::new(&task.name));
        row.add_cell(Cell::new(&task.priority.to_string()));
        row.add_cell(Cell::new(if task.done { "Y" } else { "N" }));
        if let Some(due_date) = task.due_date {
            row.add_cell(Cell::new(&due_date.to_string()));
        }
        table.add_row(row);
    }
    table.to_string()
}

/// Displays the task ID, assignee, name, priority, and due date in a table.
/// Requires access to the Tri controller to be able to lookup names.
pub fn render_everybodys_tasks(tasks: Vec<Task>, tri: &Tri) -> String {
    let users = tasks.iter().map(|task| task.user_id).collect::<Vec<_>>();
    let users = users
        .into_par_iter()
        .filter_map(|id| tri.name_for_id(id, true).map(|name| (id, name)).ok())
        .collect::<HashMap<_, _>>();

    let mut table = Table::new();
    table.set_titles(Row::from(
        &["ID", "Assignee", "Name", "Priority", "Due Date"],
    ));
    for task in tasks {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&task.id.to_string()));
        row.add_cell(Cell::new(if let Some(name) = users.get(&task.user_id) {
            name
        } else {
            ""
        }));
        row.add_cell(Cell::new(&task.name));
        row.add_cell(Cell::new(&task.priority.to_string()));
        if let Some(due_date) = task.due_date {
            row.add_cell(Cell::new(&due_date.to_string()));
        }
        table.add_row(row);
    }
    table.to_string()
}

/// Displays the task ID, name, priority, and due date in a table.
pub fn render_my_tasks(tasks: Vec<Task>) -> String {
    let mut table = Table::new();
    table.set_titles(Row::from(&["ID", "Name", "Priority", "Due Date"]));
    for task in tasks {
        let mut row = Row::empty();
        row.add_cell(Cell::new(&task.id.to_string()));
        row.add_cell(Cell::new(&task.name));
        row.add_cell(Cell::new(&task.priority.to_string()));
        if let Some(due_date) = task.due_date {
            row.add_cell(Cell::new(&due_date.to_string()));
        }
        table.add_row(row);
    }
    table.to_string()
}
