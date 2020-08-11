impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        if start == destination {
            0
        } else {
            /* let total = distance.iter().sum();
            let mut part: i32 = 0;
            if start < destination {
                part = distance[start as usize..destination as usize].iter().sum();
            } else {
                part = distance[destination as usize..start as usize].iter().sum();
            }

            if 2 * part < total {
                part
            } else {
                total - part
            } */

            let (left, right) = (
                start.min(destination) as usize,
                start.max(destination) as usize,
            );

            let total = distance.iter().sum();
            let d1 = distance[left..right].iter().sum();
            d1.min(total - d1)
        }
    }
}
