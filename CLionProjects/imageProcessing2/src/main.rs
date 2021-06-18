extern crate image;
use image::{GenericImageView, Pixel, DynamicImage};

#[derive(Copy, Clone)]
struct Point{
    x:i32,
    y:i32,
}
//-------------------------------------------------------------
impl Point{
    fn distance(&self,point:Point) -> f64 {
        ((((self.x-point.x)*(self.x-point.x)) +
            ((self.y-point.y)*(self.y-point.y))) as f64).sqrt()
    }
    fn new(x:i32,y:i32)->Point{
        Point{x,y}
    }
}
//--------------------------------------------------------------
struct Shape{
    points: Vec<Point>,
}

pub struct Image{
    img: DynamicImage,
    vec_shapes: Vec<Shape>,
    dimension: (u32,u32),
}

impl Image{
    fn open(&mut self, path:&str){
        self.img = image::open(path).unwrap();
        self.dimension = self.img.dimensions();
        self.analyse();
    }

    fn get_coordinates(&self, shape_num:usize, pix_number:usize)->(i32,i32){
        let mut pt = Point::new(0,0);
        let mut coord:(i32,i32) = (0,0);
        pt = self.vec_shapes[shape_num].points[pix_number];
        coord.0 = pt.x;
        coord.1 = pt.y;
        coord
    }

    fn is_border(&self, x:u32, y:u32)->bool{
        if (x<=0) || (x>=self.img.dimensions().0 ) || (y<=0) || (y>=self.img.dimensions().1 ){
            true
        }else{
            if (self.img.get_pixel(x-1,y).to_luma()[0]==0) &&
                (self.img.get_pixel(x ,y-1 ).to_luma()[0]==0) &&
                (self.img.get_pixel(x+1 ,y ).to_luma()[0]==0) &&
                (self.img.get_pixel(x ,y+1 ).to_luma()[0]==0) {
                false
            }else{
                true
            }
        }
    }

    fn perimeter(&self, shape_num:usize)->Option<u32> {
        let mut perim:u32 = 0;
        if shape_num + 1 > self.vec_shapes.len() {
            None
        }else{
            for pixel in 0..self.vec_shapes[shape_num].points.len(){
                let (x,y) = self.get_coordinates(shape_num,pixel);
                if self.is_border(x as u32,y as u32){
                    perim+=1;
                }
            }
            Some(perim)
        }
    }

    fn analyse(&mut self){
        for y in 0..self.dimension.1{
            for x in 0..self.dimension.0{
                if self.img.get_pixel(x,y).to_luma()[0]==0{
                    let pt = Point::new(x as i32,y as i32); //Point{x:x as i32, y:y as i32};
                    let mut not_hosted:bool = true; // точка не добавлена
                    if self.vec_shapes.len()==0{
                        self.vec_shapes.push(Shape{points: Vec::new()});
                        self.vec_shapes[0].points.push(pt);
                        not_hosted = false;
                    }
                    if not_hosted{
                        for shape_index in 0..self.vec_shapes.len(){ // по всем фигурам
                            if self.vec_shapes[shape_index].is_neighbour(pt){ // если сосед, то добавляем
                                self.vec_shapes[shape_index].points.push(pt);
                                not_hosted = false; // точка добавлена
                            }
                        }
                    }
                    if not_hosted{
                        self.vec_shapes.push(Shape{points: Vec::new()});
                        let shape_len = self.vec_shapes.len();
                        self.vec_shapes[shape_len-1].points.push(pt);
                    }
                }
            }
        }
    }
}

//-------------------------------------------------------------
impl Shape{
    fn area(self:&Shape)->u32{
        self.points.len() as u32
    }
    fn is_neighbour(&self,point:Point)->bool {
        if self.points.len() == 0 {
            return false;
        } else {
            for index in 0..self.points.len(){
                if ((point.x - self.points[index].x).abs() <= 1) &&
                    ((point.y - self.points[index].y).abs() <= 1) {
                    return true;
                }
            }
        }
        return false;
    }

    fn centroid(&self)->(u32,u32){
        let  (mut x,mut y):(f64,f64) = (0.0,0.0);
        let cnt:usize = self.points.len();
        for index in 0..cnt{
            x+=self.points[index].x as f64;
            y+=self.points[index].y as f64;
        }
        x = (x/cnt as f64).round();
        y = (y/cnt as f64).round();
        (x as u32, y as u32)
    }
}
//-------------------------------------------------------------

fn main() {
    let mut image = Image{img:DynamicImage::new_bgr8(8,8),
        vec_shapes: Vec::new(),
        dimension : (0,0),
    };
    image.open("board.png");
    println!("Dimensions {:?}", image.dimension);
    println!("shapes {}", image.vec_shapes.len());
    for index in 0..image.vec_shapes.len(){
        println!("num {} perimeter {} area {} centroid {:?}",
                 index+1,
                 image.perimeter(index).unwrap(),
                 image.vec_shapes[index].area(),
                 image.vec_shapes[index].centroid());
    }
}

