mod line_reader;
use line_reader::{read_val, read_vec};

fn main() {
    read_val::<usize>();
    let starts = read_vec::<usize>();
    let ends = read_vec::<usize>();

    let (chosen_tasks_number, chosen_tasks_start_times) = select_tasks(starts, ends);
    println!("tasks number: {} ({:?})", chosen_tasks_number, chosen_tasks_start_times);
}

fn select_tasks(start_times: Vec<usize>, end_times: Vec<usize>) -> (usize, Vec<usize>) {
    let mut tasks = start_times.iter().zip(end_times.iter()).map(|(s,t)| (*s, *t)).collect::<Vec<(usize, usize)>>();
    tasks.sort_by_key(|task| task.1);

    let mut chosen_tasks_number = 0;
    let mut chosen_tasks_start_times = Vec::<usize>::new();
    let mut next_available_time = 0;
    for task in tasks {
        if task.0 >= next_available_time {
            chosen_tasks_number = chosen_tasks_number + 1;
            chosen_tasks_start_times.push(task.0);
            next_available_time = task.1 + 1;
        }
    }

    return (chosen_tasks_number, chosen_tasks_start_times);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_select_tasks() {
        let start_times = vec![1,2,4,6,8];
        let end_times = vec![3,5,7,9,10];

        let (chosen_tasks_number, chosen_tasks_start_times) = select_tasks(start_times, end_times);
        assert_eq!(chosen_tasks_number, 3);
        for correct_task_start_time in vec![1,4,8] {
            assert!(chosen_tasks_start_times.contains(&correct_task_start_time));
        }
    }
}