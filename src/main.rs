#![allow(dead_code, unused_variables, unused_mut)]

use std::fmt::Display;

/// Entry point for Chapter 10.3: Validating References with Lifetimes
///
/// Lifetimes describe how long references remain valid and how references in
/// function inputs and outputs are related. Rust checks these relationships
/// at compile time; annotations do not extend the life of any value.
fn main() {
    scope_limits_borrows();
    returning_a_borrowed_resource();
    returning_from_one_input();
    structs_borrow_course_material();
    lifetime_elision_in_a_parser();
    methods_return_borrows();
    static_course_labels();
    lifetimes_with_generics_and_traits();
}

/// # A Borrow Cannot Outlive Its Value
/// - A reference is only useful while the value it points to is still alive.
/// - The borrow checker rejects a value that would leave a reference dangling.
fn scope_limits_borrows() {
    println!("\n{:=>80}", "");
    println!("scope_limits_borrows()\n");

    // Uncomment to see the borrow checker reject a reference escaping the
    // scope that owns its data:
    //
    // let reminder;
    // {
    //     let temporary = String::from("bring the lab key");
    //     reminder = temporary.as_str();
    // }
    // println!("{reminder}");
    //
    // error[E0597]: `temporary` does not live long enough

    let reminder;
    let stored_notice = String::from("bring the lab key");
    {
        reminder = stored_notice.as_str();
        println!("Inside the valid scope: {reminder}");
    }
    println!("The owner still exists, so the borrow remains valid: {reminder}");
}

#[derive(Debug)]
struct LibraryResource {
    title: String,
    replacement_cost: u32,
}

/// Pick the less expensive resource. The returned borrow cannot outlive either
/// resource reference supplied by the caller.
fn cheaper_resource<'a>(
    left: &'a LibraryResource,
    right: &'a LibraryResource,
) -> &'a LibraryResource {
    if left.replacement_cost <= right.replacement_cost {
        left
    } else {
        right
    }
}

/// # Relating Function Inputs and Outputs
/// - `&'a LibraryResource` is a reference with lifetime `'a`.
/// - Using `'a` for both parameters and the return value means the result
///   borrows one of the inputs and is valid only while both inputs are valid.
/// - The annotation does not say which resource is returned; the function
///   body decides that, and the compiler enforces the lifetime contract.
fn returning_a_borrowed_resource() {
    println!("\n{:=>80}", "");
    println!("returning_a_borrowed_resource()\n");

    let microscope = LibraryResource {
        title: String::from("Student microscope"),
        replacement_cost: 240,
    };
    {
        let adapter = LibraryResource {
            title: String::from("USB microscope adapter"),
            replacement_cost: 35,
        };
        let item = cheaper_resource(&microscope, &adapter);
        println!(
            "Replace first: {} ({} euros)",
            item.title, item.replacement_cost
        );
    }

    // The returned borrow is constrained by both arguments. Even though the
    // microscope is still alive, the result might have referred to `adapter`:
    //
    // let item;
    // {
    //     let adapter = LibraryResource {
    //         title: String::from("USB microscope adapter"),
    //         replacement_cost: 35,
    //     };
    //     item = cheaper_resource(&microscope, &adapter);
    // }
    // println!("{}", item.title); // `adapter` has been dropped.
}

struct Course {
    code: String,
    title: String,
}

/// Return the course's code. The catalog query does not affect the returned
/// borrow because the result is taken only from `course`.

// Imagine the `_catalog_query` is used in the function for something.
fn course_code<'a>(course: &'a Course, _catalog_query: &str) -> &'a str {
    &course.code
}

/// # A Return Value Need Not Borrow Every Input
/// - The lifetime on `course_code` connects the result to `course`.
/// - `_catalog_query` has its own inferred lifetime, unrelated to the result.
/// - Signatures should describe actual relationships, not add constraints
///   between values that do not need to be related.
fn returning_from_one_input() {
    println!("\n{:=>80}", "");
    println!("returning_from_one_input()\n");

    let course = Course {
        code: String::from("IC00BJ79"),
        title: String::from("Rust Programming"),
    };

    let code;
    {
        let query = String::from("courses starting with IC");
        code = course_code(&course, &query);
    }
    println!("Course {} has code {code}", course.title);

    // If the result were incorrectly tied to both inputs, the code would be
    // rejected after `query` goes out of scope. In fact, it only borrows from
    // `course`.
}

#[derive(Debug)]
struct ReadingTask<'a> {
    chapter_title: &'a str,
    required_pages: &'a str,
}

/// # Structs That Borrow Data
/// - A struct holding references needs lifetime parameters on its definition.
/// - Each `ReadingTask<'a>` must not outlive the strings referenced by its
///   fields.
fn structs_borrow_course_material() {
    println!("\n{:=>80}", "");
    println!("structs_borrow_course_material()\n");

    let chapter = String::from("Managing Growing Projects");
    let page_range = String::from("pages 339-352");
    let task = ReadingTask {
        chapter_title: &chapter,
        required_pages: &page_range,
    };
    println!("Read {} ({})", task.chapter_title, task.required_pages);

    // Both source strings must remain alive while `task` is used.
    // let task;
    // {
    //     let chapter = String::from("Managing Growing Projects");
    //     let page_range = String::from("pages 339-352");
    //     task = ReadingTask {
    //         chapter_title: &chapter,
    //         required_pages: &page_range,
    //     };
    // }
    // println!("{task:?}");
}

/// Return the suffix after the final dot, borrowing it from the supplied name.
fn file_suffix(file_name: &str) -> &str {
    match file_name.rfind('.') {
        Some(dot) => &file_name[dot + 1..],
        None => "",
    }
}

/// # Lifetime Elision in a Parser
/// - In a function with one reference input, Rust infers that a returned
///   reference is borrowed from that input.
/// - `file_suffix` is equivalent to
///   `fn file_suffix<'a>(file_name: &'a str) -> &'a str`.
/// - With multiple reference inputs and a returned reference, Rust needs an
///   explicit relationship if it cannot infer one from the method rules.
fn lifetime_elision_in_a_parser() {
    println!("\n{:=>80}", "");
    println!("lifetime_elision_in_a_parser()\n");

    let uploaded_file = String::from("assignment-final.rs");
    println!(
        "Uploaded Rust source suffix: {}",
        file_suffix(&uploaded_file)
    );
    println!("No suffix: '{}'", file_suffix("README"));

    // There is no rule that chooses which input a returned reference belongs
    // to in this free function. Uncomment to see the compiler error:
    //
    // fn choose_file_name(first: &str, second: &str) -> &str {
    //     if first.len() < second.len() { first } else { second }
    // }
}

impl<'a> ReadingTask<'a> {
    /// Return the chapter title borrowed from this task.
    fn title(&self) -> &str {
        self.chapter_title
    }

    /// Return the required page range borrowed from this task.
    fn pages(&self) -> &str {
        self.required_pages
    }
}

/// # Lifetimes in Methods
/// - The lifetime parameter after `impl` is the one used by the struct.
/// - The elision rules tie a method's returned reference to the borrow of
///   `self`, so the returned text cannot outlive the task.
fn methods_return_borrows() {
    println!("\n{:=>80}", "");
    println!("methods_return_borrows()\n");

    let chapter = String::from("Managing Growing Projects");
    let pages = String::from("pages 339-352");
    let task = ReadingTask {
        chapter_title: &chapter,
        required_pages: &pages,
    };
    let title = task.title();
    println!("{}: {}", title, task.pages());
}

/// # The `'static` Lifetime
/// - A `'static` reference is valid for the entire program.
/// - String literals and constants that refer to them have this lifetime.
/// - Adding `'static` cannot extend a locally owned `String`'s lifetime.
fn static_course_labels() {
    println!("\n{:=>80}", "");
    println!("static_course_labels()\n");

    const COURSE_LABEL: &str = "Rust Programming / IC00BJ79";
    let label: &'static str = COURSE_LABEL;
    println!("Course label available throughout the program: {label}");

    // The owned text is temporary and cannot be borrowed for `'static`:
    //
    // let local_label = String::from("Rust Programming");
    // let program_label: &'static str = local_label.as_str();
}

/// Print a displayable status and return the course code borrowed from `course`.
fn report_course<'a, T>(course: &'a Course, status: T) -> &'a str
where
    T: Display,
{
    println!("{}: {status}", course.title);
    &course.code
}

/// # Combining Lifetimes, Type Parameters, and Trait Bounds
/// - The lifetime `'a` connects the returned code to the borrowed `Course`.
/// - `T` is a separate generic type parameter.
/// - `T: Display` allows the status to be printed without requiring a
///   particular status type.
fn lifetimes_with_generics_and_traits() {
    println!("\n{:=>80}", "");
    println!("lifetimes_with_generics_and_traits()\n");

    let course = Course {
        code: String::from("IC00BJ79"),
        title: String::from("Rust Programming"),
    };
    let code = report_course(&course, "enrollment is open");
    println!("Reference returned for course code: {code}");
}
