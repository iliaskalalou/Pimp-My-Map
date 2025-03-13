#[derive(Clone)]
pub struct Cell
{
    pub options: Vec<usize>,
    pub collapsed: bool,
}

impl Cell
{
    pub fn new(options: Vec<usize>) -> Cell
    {
        Cell {
            options,
            collapsed: false,
        }
    }
}
