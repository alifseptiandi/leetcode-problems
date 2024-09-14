impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        let values = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        
        let mut remainder = num;
        
        for (value, symbol) in values {
            while remainder >= value {
                result.push_str(symbol);
                remainder -= value;
            }
        }
        
        result
    }
}