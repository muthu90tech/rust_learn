impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut rev: i32 = 0;
        let mut cx = x;
        let mut flag = false;
        if (x < 0) {
            cx = -cx;
            flag = true;
        }
        
        while (cx > 0) {
            let last = cx % 10;
            cx = cx / 10;
            // Check for overflow before multiplying and adding
            if (rev > i32::MAX / 10 || (rev == i32::MAX / 10 && last > 7)) {
                return 0;
            }
            rev = rev * 10 + last;
        
        }
        if (flag) {
            rev = -rev;
        }
        rev
    }
}
