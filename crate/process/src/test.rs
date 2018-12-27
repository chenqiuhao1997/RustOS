#[test]
fn it_works() {
	assert_eq!(2 + 2, 4);
}

#[test]
fn test_event_hub() {
	use event_hub::EventHub;
	let mut eh = EventHub::<usize>::new();
	eh.push(3, 100);
	eh.push(5, 200);
	eh.push(9, 300);
	eh.push(9, 400);
	eh.push(9, 500);
	eh.push(13, 600);
	eh.push(15, 700);
	eh.push(17, 800);
	eh.push(19, 900);

	for _i in 0..3 {
		assert_eq!(eh.pop(), None);
		eh.tick();
	}
	assert_eq!(eh.pop(), Some(100));

	eh.remove(400);

	eh.tick();
	assert_eq!(eh.pop(), None);
	eh.tick();
	assert_eq!(eh.pop(), Some(200));

	for _i in 5..9 {
		assert_eq!(eh.pop(), None);
		eh.tick();
	}
	let data9_1 = eh.pop();
	eh.remove(600);
	let data9_2 = eh.pop();
	let expect9 = (Some(300), Some(500));
	assert!(expect9 == (data9_1, data9_2) || expect9 == (data9_2, data9_1));

	for _i in 9..15 {
		assert_eq!(eh.pop(), None);
		eh.tick();
	}
	eh.tick();
	eh.tick();
	eh.tick();
	assert_eq!(eh.pop(), Some(700));
	assert_eq!(eh.pop(), Some(800));
	assert_eq!(eh.pop(), None);
	eh.tick();
	assert_eq!(eh.pop(), Some(900));
	assert_eq!(eh.pop(), None);
}
