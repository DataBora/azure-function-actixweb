use chrono::NaiveDate;
use crate::model::models::SalesOrders;

//Function to change Date Format in Sales Orders file from yyyy-mm-dd to yyyymmdd
pub fn convert_date_format(sales_order: &mut SalesOrders)-> Result<(), String>{
    
    sales_order.order_date = match reformat_date(&sales_order.order_date) {
        Ok(date) => date,
        Err(err) => return Err(format!("Error reformatting date: {}", err)),
    };

    sales_order.delivery_date = match reformat_date(&sales_order.delivery_date) {
        Ok(date) => date,
        Err(err) => return Err(format!("Error reformatting date: {}", err)),
    };
    
    Ok(())
}

fn reformat_date(date: &str) -> Result<String, String> {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(parsed_date) => Ok(parsed_date.format("%Y%m%d").to_string()),
        Err(_) => Err(format!("Invalid date format: {}", date)),
    }
}
