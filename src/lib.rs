use colored::*;
pub use colored::Color;

const MAP : &'static str = r###"   180   150W  120W  90W   60W   30W   000   30E   60E   90E   120E  150E  180
    |     |     |     |     |     |     |     |     |     |     |     |     |
90N-+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-90N
    |           . _..::__:  ,-"-"._        |7       ,     _,.__             |
    |   _.___ _ _<_>`!(._`.`-.    /         _._     `_ ,_/  '  '-._.---.-.__|
    |>.{     " " `-==,',._\{  \  / {)      / _ ">_,-' `                mt-2_|
60N-+  \_.:--.       `._ )`^-. "'       , [_/(                       __,/-' +-60N
    | '"'     \         "    _L        oD_,--'                )     /. (|   |
    |          |           ,'          _)_.\\._<> 6              _,' /  '   |
    |          `.         /           [_/_'` `"(                <'}  )      |
30N-+           \\    .-. )           /   `-'"..' `:._          _)  '       +-30N
    |    `        \  (  `(           /         `:\  > \  ,-^.  /' '         |
    |              `._,   ""         |           \`'   \|   ?_)  {\         |
    |                 `=.---.        `._._       ,'     "`  |' ,- '.        |
000-+                   |    `-._         |     /          `:`<_|h--._      +-000
    |                   (        >        .     | ,          `=.__.`-'\     |
    |                    `.     /         |     |{|              ,-.,\     .|
    |                     |   ,'           \   / `'            ,"     \     |
30S-+                     |  /              |_'                |  __  /     +-30S
    |                     | |                                  '-'  `-'   \.|
    |                     |/                                         "    / |
    |                     \.                                             '  |
60S-+                                                                       +-60S
    |                      ,/            ______._.--._ _..---.---------._   |
    |     ,-----"-..?----_/ )      __,-'"             "                  (  |
    |-.._(                  `-----'                                       `-|
90S-+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-90S
    Map 1998 Matthew Thomas.|Freely usable as long as this|line is included.|
    |     |     |     |     |     |     |     |     |     |     |     |     |
   180   150W  120W  90W   60W   30W   000   30E   60E   90E   120E  150E  180"###;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub color: Option<colored::Color>,
    pub name: Option<String>,
    pub point : Option<String>
}

impl Point {
    pub fn transform(&self, from: &Rect, to: &Rect) -> Point {
        Point {
            x: (((self.x - from.x) / from.w) * to.w) + to.x,
            y: (((self.y - from.y) / from.h) * to.h) + to.y,
            color: self.color,
            name: self.name.clone(),
            point: self.point.clone()
        }
    }
}

pub struct Plot {
    // This contains the ascii art we are plotting against.
    map: String,

    // Map bounds to plot the points into. Plotted points are transformed into these bounds using point coordinates and a simple coordinate transform.
    map_coordinates: Rect,

    /// Refers to the coordinate space for the point we are plotting.
    /// For GPS, use `space_plot::gps_coordspace`.
    point_coordinates: Rect
}

static gps_coordspace : Rect = Rect {
    x: -180.0,
    y: 90.0,
    w: 360.0,
    h: -180.0
};

impl Plot {
    pub fn default() -> Plot {
        Plot {
            map: MAP.to_string(),
            map_coordinates: Rect {
                x: 4.0,
                y: 2.0,
                w: 70.0,
                h: 22.0
            },
            point_coordinates: gps_coordspace
        }
    }

    pub fn make_blank(width: usize, height: usize) -> Plot {
        let mut lines : Vec<String> = vec![];
        for i in 0..height {
            lines.push(std::iter::repeat(" ").take(width).collect::<String>());
        }

        let string = lines.join("\n");

        return Plot {
            map: string,
            map_coordinates: Rect {
                x: 0.0,
                y: 0.0,
                w: width as f64,
                h: height as f64
            },
            point_coordinates: gps_coordspace
        }
    }
}


pub fn render_point (points: Vec<Point>, plot: Plot) -> String {

    let lines : Vec<&str> = plot.map.split("\n").into_iter().collect::<Vec<&str>>();
    let mut rows = lines.iter().map(|v| v.chars().into_iter().map(|v| format!("{}", v).white().dimmed().to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    
    for point in points {
        let coord = point.transform(&plot.point_coordinates, &plot.map_coordinates);
        let x = coord.x.abs() as usize;
        let y = coord.y.abs() as usize;

        if y >= rows.len() || x >= rows[0].len() {
            println!("skipping output coord: {} {} because it's out of bounds", x, y);
            continue;
        }

        let icon : String;

        if let Some(string) = point.point {
            icon = string.clone();
        } else {
            icon = "*".to_string();
        }



        if let Some(color) = point.color {
            rows[y][x] = icon.color(color).bold().to_string();
        } else {
            rows[y][x] = icon.white().bold().to_string();
        }

        if let Some(name) = point.name {
            for (i, character) in name.chars().into_iter().enumerate() {
                let pos = x + 2 + i;
                if pos < rows[y].len() {
                    rows[y][pos] = format!("{}", character);
                }
            }
        }
        
    }

    return rows.iter().map(|v| v.join("")).collect::<Vec<String>>().join("\n");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_coord_transform() {
        let point = Point {
            x: 50.0,
            y: 50.0,
            point: None,
            color: None,
            name: None
        };

        assert_eq!(point.transform(&Rect {
            x: 0.0, y: 0.0, w: 100.0, h: 100.0 
        }, &Rect {
            x: 0.0, y: 0.0, w: 1.0, h: 1.0
        }), Point {
            x: 0.5,
            y: 0.5,
            point: None,
            color: None,
            name: None
        });

        assert_eq!(point.transform(&Rect {
            x: 50.0, y: 50.0, w: 100.0, h: 100.0 
        }, &Rect {
            x: 0.0, y: 0.0, w: 1.0, h: 1.0
        }), Point {
            x: 0.0,
            y: 0.0,
            point: None,
            color: None,
            name: None
        });

        assert_eq!(point.transform(&Rect {
            x: 100.0, y: 100.0, w: 100.0, h: 100.0 
        }, &Rect {
            x: 0.0, y: 0.0, w: 1.0, h: 1.0
        }), Point {
            x: -0.5,
            y: -0.5,
            point: None,
            color: None,
            name: None
        });

        assert_eq!(point.transform(&Rect {
            x: 0.0, y: 0.0, w: 50.0, h: 50.0 
        }, &Rect {
            x: 0.0, y: 0.0, w: 1.0, h: 1.0
        }), Point {
            x: 1.0,
            y: 1.0,
            point: None,
            color: None,
            name: None
        });


    }
}
