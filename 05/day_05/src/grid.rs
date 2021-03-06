use crate::vent::*;

pub struct Grid {
    cells: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(x_dim: usize, y_dim: usize) -> Self{
        let cells = vec![vec![0; x_dim]; y_dim];
        Self{
            cells,
        }
    }
    pub fn get_cell(&self, pt: (usize, usize)) -> Option<usize>{
        match (self.cells.get(pt.1))?.get(pt.0){
            Some(num) => Some(*num),
            None      => None
        }
    }
    pub fn get_cell_mut(&mut self, pt: (usize, usize)) -> Option<&mut usize>{
        self.cells
            .get_mut(pt.1)?
            .get_mut(pt.0)
    }
    pub fn increment_cell(&mut self, pt:(usize, usize)) {
        let cell = self.get_cell_mut(pt).expect("Cell not found");
        *cell += 1;
    }
    pub fn increment_vent(&mut self, vent: Vent) {
        let points = vent.get_points();
        for point in points{
            self.increment_cell(point);
        }
    }

    #[cfg(test)]
    fn print_cells(&self){
        for row in self.cells.iter(){
            println!("{:?}", row);
        }
    }

    pub fn count_intersections(&self) -> usize {
        #[cfg(test)]
        self.print_cells();

        self.cells
            .iter()
            .map(|row| -> usize{
                row
                    .iter()
                    .filter(|x| **x > 1)
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_creation(){
        let grid = Grid::new(10,15);
        assert_eq!(grid.get_cell((9,14)), Some(0));
    }
    #[test]
    fn test_increment(){
        let mut grid = Grid::new(10,15);
        grid.increment_cell((5,4));
        assert_eq!(grid.get_cell((5,4)), Some(1));
    }
    #[test]
    fn test_vent_increment(){
        let mut grid = Grid::new(10,15);
        let vent = Vent::new((2,2), (4,4));
        grid.increment_vent(vent);
        assert_eq!(grid.get_cell((2,2)), Some(1));
        assert_eq!(grid.get_cell((3,3)), Some(1));
        assert_eq!(grid.get_cell((4,4)), Some(1));
        assert_eq!(grid.get_cell((4,5)), Some(0));
    }
}
