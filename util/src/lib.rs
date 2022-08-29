pub fn get_page_buffer<T>(collection: &Vec<&T>, page: usize, page_size: usize) -> Vec<T>
where
    T: Clone,
{
    let offset = page * page_size;
    let endpoint = offset + page_size;

    let total_elements = collection.len();

    let clone_elements = |element: &&T| (*element).clone();

    if offset > total_elements {
        Vec::<T>::new()
    } else if endpoint > total_elements {
        let buffer = &collection[offset..total_elements];
        let cloned_buffer: Vec<T> = buffer.iter().map(clone_elements).collect();

        cloned_buffer
    } else {
        let buffer = &collection[offset..endpoint];
        let cloned_buffer: Vec<T> = buffer.iter().map(clone_elements).collect();

        cloned_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_smaller_than_page_size() {
        let page_size = 100;
        let page_number = 0;

        let test_string = String::from("test");

        let room_names = vec![&test_string, &test_string];

        let actual: Vec<String> = get_page_buffer(&room_names, page_number, page_size);

        assert_eq!(actual.len(), room_names.len());
        assert_eq!(
            actual,
            room_names
                .iter()
                .map(|string_ref| { String::from(*string_ref) })
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_source_larger_than_page_size() {
        let page_size = 2;
        let page_number = 0;

        let test_string = String::from("test");

        let room_names = vec![&test_string, &test_string, &test_string];

        let actual: Vec<String> = get_page_buffer(&room_names, page_number, page_size);

        assert_eq!(actual.len(), page_size);
    }

    #[test]
    fn test_page_number_plus_size_greater_than_source() {
        let page_size = 2;
        let page_number = 1;

        let test_string = String::from("test");

        let room_names = vec![&test_string, &test_string, &test_string];

        let actual: Vec<String> = get_page_buffer(&room_names, page_number, page_size);

        // only get one element, because page 0 includes first two elements
        assert_eq!(actual.len(), 1);
    }

    #[test]
    fn test_invalid_page_number() {
        let page_size = 2;
        let page_number = 10;

        let test_string = String::from("test");

        let room_names = vec![&test_string, &test_string, &test_string];

        let actual: Vec<String> = get_page_buffer(&room_names, page_number, page_size);

        // should get an empty vector, because there is nothing at page 10
        assert_eq!(actual.len(), 0);
    }
}
