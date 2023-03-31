pub trait Chart {
    fn display(&self) -> String;
}

pub enum ChartType {
    LineChart,
    PieChart,
    BarChart,
}

pub struct LineChart {}

impl Chart for LineChart {
    fn display(&self) -> String {
        println!("line chart");
        "line chart".to_string()
    }
}

pub struct PieChart {}

impl Chart for PieChart {
    fn display(&self) -> String {
        println!("pie chart");

        "pie chart".to_string()
    }
}

pub struct BarChart {}

impl Chart for BarChart {
    fn display(&self) -> String {
        println!("bar chart");

        "bar chart".to_string()
    }
}

pub struct ChartFactory {}

impl ChartFactory {
    pub fn get_chart(chart_type: ChartType) -> Box<dyn Chart> {
        match chart_type {
            ChartType::LineChart => Box::new(LineChart {}),
            ChartType::PieChart => Box::new(PieChart {}),
            ChartType::BarChart => Box::new(BarChart {}),
        }
    }
}

pub fn get_chart(chart_type: ChartType) -> Box<dyn Chart> {
    match chart_type {
        ChartType::LineChart => Box::new(LineChart {}),
        ChartType::PieChart => Box::new(PieChart {}),
        ChartType::BarChart => Box::new(BarChart {}),
    }
}

#[cfg(test)]
mod test {
    use super::{get_chart, ChartFactory, ChartType};

    #[test]
    fn get_chart_test() {
        let line_chart = get_chart(ChartType::LineChart);
        let pie_chart = ChartFactory::get_chart(ChartType::PieChart);
        let bar_chart = ChartFactory::get_chart(ChartType::BarChart);

        assert_eq!("line chart", line_chart.display());
        assert_eq!("pie chart", pie_chart.display());
        assert_eq!("bar chart".to_string(), bar_chart.display());
        // assert_eq!("bar", String::from("bar"));
    }
}
