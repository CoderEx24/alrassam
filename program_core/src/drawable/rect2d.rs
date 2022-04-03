#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_make_square() {
        let sqr = Rect2::new_square(Vector2::new(10.0, 0.0), 10, None, None, None);

        assert_eq!(Vector2::new(10.0, 0.0), sqr.start());
        assert_eq!(Vector2::new(0.0, 10.0), sqr.end());
    }

    #[test]
    fn test_translate() {
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        rect.translate(Vector2::new(2.0, 2.0));

        assert_eq!(Vector2::new(2.0, 2.0), rect.start());
        assert_eq!(Vector2::new(3.0, 3.0), rect.end());
    }

    #[test]
    fn test_rotate() {
        use core::f64::consts::{ FRAC_PI_4, FRAC_PI_2 };

        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        rect.rotate(FRAC_PI_4);
        assert_eq!(FRAC_PI_2, rect.angle());
        assert_eq!(Vector2::new(0.0, 0.0), rect.start());
        assert_eq!(Vector2::new(0.0, 2f64.sqrt()), rect.end());
    }

    #[test]
    fn test_scale() {
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        rect.scale(2.0);

        assert_eq!(Vector2::new(0.0, 0.0), rect.start());
        assert_eq!(Vector2::new(2.0, 2.0), rect.end());

    }

    #[test]
    fn test_contains() {
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), None, None, None);

        let v_inside = [Vector2::new(0.5, 0.5), Vector2::new(0.75, 0.0), Vector2::new(1.0, 0.0), Vector2::new(1.0, 1.0)];
        let v_outside = [Vector2::new(2.0, 2.0), Vector2::new(1.0, 1.1), Vector2::new(0.0, 5.0), Vector2::new(12.0, 16.4)];

        assert_eq!(rect.contains(v_inside[0]));
        assert_eq!(rect.contains(v_inside[1]));
        assert_eq!(rect.contains(v_inside[2]));
        assert_eq!(rect.contains(v_inside[3]));
        
        assert_eq!(!rect.contains(v_outside[0]));
        assert_eq!(!rect.contains(v_outside[1]));
        assert_eq!(!rect.contains(v_outside[2]));
        assert_eq!(!rect.contains(v_outside[3]));
    }

}
