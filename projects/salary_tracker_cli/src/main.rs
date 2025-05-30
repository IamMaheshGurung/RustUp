use std::env;
use sqlx::PgPool;
use dotenvy::dotenv;
use chrono::{DateLike, Duration, Utc, Weekday, NaiveDate};
use std::io;
use genpdf::{self, elements, style};


#[tokio::main]
async fn main() -> Result <(), sqlx::Error> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "summary" {
        return show_weekly_summary(&pool).await;
    } else if args.len() > 1 && args[1] == "pdf" {
        return gen_weekly_pdf(&pool).await;
        
        
    }

    //Pompt for getting today date
    println!("Please enter today's date (YYYY-MM-DD):");
    let mut date_input = String::new();
    io::stdin()
        .read_line(&mut date_input)
        .unwrap();
        

   let work_date = match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => {
        eprintln!("❌ Invalid date format. Please use YYYY-MM-DD.");
        return Ok(());
    }
};

    // Prompt for Job A hours
    println!("Enter hours of shift in **LALACA**:");
    let mut job_a_input = String::new();
    io::stdin().read_line(&mut job_a_input).unwrap();
    let job_a_hours: f64 = job_a_input.trim().parse().expect("Please enter a valid number");

    // Prompt for Job B hours
    println!("Enter hours shift in **GORA KADAN**:");
    let mut job_b_input = String::new();
    io::stdin().read_line(&mut job_b_input).unwrap();
    let job_b_hours: f64 = job_b_input.trim().parse().expect("Please enter a valid number");

    // Insert into the database
    sqlx::query!(
        "INSERT INTO work_entries (work_date, job_a_hours, job_b_hours) VALUES ($1, $2, $3)",
        work_date,
        job_a_hours,
        job_b_hours
    )
    .execute(&pool)
    .await?;


    
    println!("Connection successful");
    
    Ok(())
    
}



async fn show_weekly_summary(pool: &PgPool) -> Result<(), sqlx::Error> {
    let today = Utc::now().date_naive();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + Duration::days(6);

    let result = sqlx::query!(
        r#"
        SELECT 
            COALESCE(SUM(job_a_hours), 0) AS sum_a, 
            COALESCE(SUM(job_b_hours), 0) AS sum_b 
        FROM work_entries 
        WHERE work_date BETWEEN $1 AND $2
        "#,
        start_of_week,
        end_of_week
    )
    .fetch_one(pool)
    .await?;

    let total_hours = result.sum_a + result.sum_b;
    println!("\n🗓 Weekly Summary: {} to {}", start_of_week, end_of_week);
    println!("📍 LALACA: {:.2} hours", result.sum_a);
    println!("📍 GORA KADAN: {:.2} hours", result.sum_b);
    println!("🧮 Total Hours: {:.2} hours", total_hours);

    let rate_a = 1300.0;
    let rate_b = 1150.0;
    let total_salary = result.sum_a * rate_a + result.sum_b * rate_b;

    println!("💴 Estimated Total Salary: ¥{:.2}", total_salary);

    Ok(())
}





async fn export_weekly_summary_to_pdf(pool: &PgPool) -> Result<(), sqlx::Error> {
    let today = Utc::now().date_naive();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + Duration::days(6);

    let result = sqlx::query!(
        r#"
        SELECT 
            COALESCE(SUM(job_a_hours), 0) AS sum_a, 
            COALESCE(SUM(job_b_hours), 0) AS sum_b 
        FROM work_entries 
        WHERE work_date BETWEEN $1 AND $2
        "#,
        start_of_week,
        end_of_week
    )
    .fetch_one(pool)
    .await?;

    let rate_a = 1300.0;
    let rate_b = 1150.0;
    let total_salary = result.sum_a * rate_a + result.sum_b * rate_b;

    // Create PDF document
    let mut doc = genpdf::Document::new(genpdf::fonts::FontFamily::default());
    doc.set_title("Weekly Salary Report");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let mut content = elements::LinearLayout::vertical();
    content.push(elements::Paragraph::new("🗓 Weekly Salary Report").styled(style::Style::new().bold()));
    content.push(elements::Break::new(1));
    content.push(elements::Paragraph::new(format!("Period: {} to {}", start_of_week, end_of_week)));
    content.push(elements::Paragraph::new(format!("LALACA Hours: {:.2}", result.sum_a)));
    content.push(elements::Paragraph::new(format!("GORA KADAN Hours: {:.2}", result.sum_b)));
    content.push(elements::Paragraph::new(format!("Total Hours: {:.2}", result.sum_a + result.sum_b)));
    content.push(elements::Paragraph::new(format!("Estimated Salary: ¥{:.2}", total_salary)));

    doc.push(content);

    doc.render_to_file("weekly_salary_report.pdf").expect("Failed to write PDF");

    println!("📄 PDF generated: weekly_salary_report.pdf");
    Ok(())
}
