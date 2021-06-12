extern crate image;
use image::{GenericImageView, Pixel, Rgb, Luma};

#[derive(Copy,Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Shape{
    points: Vec<Point>,
}

fn is_black(pix:Luma<u8>)->bool{
    pix[0] == 0
}

fn is_neighbour(point:Point, shape:&Shape)->bool{
    if (shape.points.len()==0) {
        return false;
    }else{
        for index in (0..shape.points.len()){
            if (((point.x-shape.points[index].x).abs()<=1) &&
                ((point.y-shape.points[index].y).abs()<=1)) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let img = image::open("board.png").unwrap();
    let (width, heigth) = img.dimensions();
    println!("dimensions ({},{}) ", width, heigth);
    //println!("{:?}", img.color());
    //img.save("output.png").unwrap();

    //let mut pixel1 = img.get_pixel(0,0);
    //let mut  pixel2 = img.get_pixel(50,50);
    //println!("pixel 1 {:?}", pixel1.to_rgb());
    //println!("pixel 2 {:?}", pixel2.to_rgb());

    let mut vec_shapes:Vec<Shape> = Vec::new();
    for y in (0..heigth){
        for x in (0..width){
            //println!("pixel {:?}", img.get_pixel(x,y).to_luma());
            //if (is_black(img.get_pixel(x,y).to_luma())) {println!("black");}

            if is_black(img.get_pixel(x,y).to_luma()){
                let pt = Point{x:x as i32, y:y as i32};
                let mut not_hosted:bool = true; //точка не добавлена
                if (vec_shapes.len()==0) {
                    vec_shapes.push(Shape{points:Vec::new()});
                    vec_shapes[0].points.push(pt);
                    not_hosted = false;
                }
                if (not_hosted){
                    for shape_index in (0..vec_shapes.len()){  // по всем фигурам
                        if is_neighbour(pt,&vec_shapes[shape_index]){ // если сосед, то добавлем
                            vec_shapes[shape_index].points.push(pt);
                            not_hosted = false; //точка добавлена
                        }
                    }
                }
                if (not_hosted){//ни с кем не сосед
                    vec_shapes.push(Shape{points:Vec::new()});// еще одна фигура
                    let shape_len = vec_shapes.len();
                    vec_shapes[shape_len-1].points.push(pt); // и ей добавляем точку
                }
            }
        }
    }
    println!("shapes {}", vec_shapes.len());
}


