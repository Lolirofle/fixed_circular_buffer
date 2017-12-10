use ::*;

#[test]
fn test_len(){
	let l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	assert_eq!(l.len(),4);

	let l = CircularBuffer::from(Box::new(['a','b']) as Box<[char]>);
	assert_eq!(l.len(),2);

	let l = CircularBuffer::from(Box::new(['a']) as Box<[char]>);
	assert_eq!(l.len(),1);
}

#[test]
#[should_panic]
fn test_len_empty(){
	let _ = CircularBuffer::from(Box::new([]) as Box<[char]>);
}

#[test]
fn test_queue(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	assert_eq!(l.first,0);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.queue('9');
	assert_eq!(l.first,3);
	assert_eq!(&*l.list,&['a','b','c','9']);

	l.queue('8');
	assert_eq!(l.first,2);
	assert_eq!(&*l.list,&['a','b','8','9']);

	l.queue('7');
	assert_eq!(l.first,1);
	assert_eq!(&*l.list,&['a','7','8','9']);

	l.queue('6');
	assert_eq!(l.first,0);
	assert_eq!(&*l.list,&['6','7','8','9']);

	l.queue('5');
	assert_eq!(l.first,3);
	assert_eq!(&*l.list,&['6','7','8','5']);

	l.queue('4');
	assert_eq!(l.first,2);
	assert_eq!(&*l.list,&['6','7','4','5']);
}

#[test]
fn test_set_first(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(0);
	assert_eq!(l.first,0);

	l.set_first(1);
	assert_eq!(l.first,1);

	l.set_first(1);
	assert_eq!(l.first,2);

	l.set_first(1);
	assert_eq!(l.first,3);

	l.set_first(1);
	assert_eq!(l.first,0);

	l.set_first(2);
	assert_eq!(l.first,2);

	l.set_first(4);
	assert_eq!(l.first,2);
}

#[test]
fn test_get(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(0);
	assert_eq!(l.first,0);
	assert_eq!(*l.get(0),'a');
	assert_eq!(*l.get(1),'b');
	assert_eq!(*l.get(2),'c');
	assert_eq!(*l.get(3),'d');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(1);
	assert_eq!(l.first,1);
	assert_eq!(*l.get(0),'b');
	assert_eq!(*l.get(1),'c');
	assert_eq!(*l.get(2),'d');
	assert_eq!(*l.get(3),'a');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(2);
	assert_eq!(l.first,2);
	assert_eq!(*l.get(0),'c');
	assert_eq!(*l.get(1),'d');
	assert_eq!(*l.get(2),'a');
	assert_eq!(*l.get(3),'b');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(3);
	assert_eq!(l.first,3);
	assert_eq!(*l.get(0),'d');
	assert_eq!(*l.get(1),'a');
	assert_eq!(*l.get(2),'b');
	assert_eq!(*l.get(3),'c');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(4);
	assert_eq!(l.first,0);
	assert_eq!(*l.get(0),'a');
	assert_eq!(*l.get(1),'b');
	assert_eq!(*l.get(2),'c');
	assert_eq!(*l.get(3),'d');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(5);
	assert_eq!(l.first,1);
	assert_eq!(*l.get(0),'b');
	assert_eq!(*l.get(1),'c');
	assert_eq!(*l.get(2),'d');
	assert_eq!(*l.get(3),'a');
}

#[test]
fn test_get_mut(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(0);
	assert_eq!(l.first,0);
	assert_eq!(*l.get_mut(0),'a');
	assert_eq!(*l.get_mut(1),'b');
	assert_eq!(*l.get_mut(2),'c');
	assert_eq!(*l.get_mut(3),'d');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(1);
	assert_eq!(l.first,1);
	assert_eq!(*l.get_mut(0),'b');
	assert_eq!(*l.get_mut(1),'c');
	assert_eq!(*l.get_mut(2),'d');
	assert_eq!(*l.get_mut(3),'a');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(2);
	assert_eq!(l.first,2);
	assert_eq!(*l.get_mut(0),'c');
	assert_eq!(*l.get_mut(1),'d');
	assert_eq!(*l.get_mut(2),'a');
	assert_eq!(*l.get_mut(3),'b');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(3);
	assert_eq!(l.first,3);
	assert_eq!(*l.get_mut(0),'d');
	assert_eq!(*l.get_mut(1),'a');
	assert_eq!(*l.get_mut(2),'b');
	assert_eq!(*l.get_mut(3),'c');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(4);
	assert_eq!(l.first,0);
	assert_eq!(*l.get_mut(0),'a');
	assert_eq!(*l.get_mut(1),'b');
	assert_eq!(*l.get_mut(2),'c');
	assert_eq!(*l.get_mut(3),'d');

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(5);
	assert_eq!(l.first,1);
	assert_eq!(*l.get_mut(0),'b');
	assert_eq!(*l.get_mut(1),'c');
	assert_eq!(*l.get_mut(2),'d');
	assert_eq!(*l.get_mut(3),'a');
}

#[test]
fn test_swap_at(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.swap_at(0,'0');
	assert_eq!(&*l.list,&['0','b','c','d']);

	l.swap_at(1,'1');
	assert_eq!(&*l.list,&['0','1','c','d']);

	l.swap_at(2,'2');
	assert_eq!(&*l.list,&['0','1','2','d']);

	l.swap_at(3,'3');
	assert_eq!(&*l.list,&['0','1','2','3']);

	l.swap_at(4,'4');
	assert_eq!(&*l.list,&['4','1','2','3']);

	l.swap_at(5,'5');
	assert_eq!(&*l.list,&['4','5','2','3']);

	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	l.set_first(1);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.swap_at(0,'0');
	assert_eq!(&*l.list,&['a','0','c','d']);

	l.swap_at(1,'1');
	assert_eq!(&*l.list,&['a','0','1','d']);

	l.swap_at(2,'2');
	assert_eq!(&*l.list,&['a','0','1','2']);

	l.swap_at(3,'3');
	assert_eq!(&*l.list,&['3','0','1','2']);

	l.swap_at(4,'4');
	assert_eq!(&*l.list,&['3','4','1','2']);

	l.swap_at(5,'5');
	assert_eq!(&*l.list,&['3','4','5','2']);
}

#[test]
fn test_swap_internal(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.swap_internal(0,3);
	assert_eq!(&*l.list,&['d','b','c','a']);

	l.swap_internal(3,0);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.swap_internal(1,2);
	assert_eq!(&*l.list,&['a','c','b','d']);

	l.swap_internal(2,1);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.swap_internal(0,5);
	assert_eq!(&*l.list,&['b','a','c','d']);

	l.swap_internal(5,0);
	assert_eq!(&*l.list,&['a','b','c','d']);
}

#[test]
fn test_swap(){
	let mut l = CircularBuffer::from(Box::new(['d','c','b','a']) as Box<[char]>);
	assert_eq!(&*l.list,&['d','c','b','a']);

	l.first = 0;
	l.swap('0');
	assert_eq!(&*l.list,&['0','c','b','a']);

	l.first = 1;
	l.swap('1');
	assert_eq!(&*l.list,&['0','1','b','a']);

	l.first = 2;
	l.swap('2');
	assert_eq!(&*l.list,&['0','1','2','a']);

	l.first = 3;
	l.swap('3');
	assert_eq!(&*l.list,&['0','1','2','3']);
}

#[test]
fn test_iter(){
	let l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,0)};
	let mut i = l.iter();

	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert!(i.next().is_none());

	let l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,1)};
	let mut i = l.iter();

	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert!(i.next().is_none());

	let l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,2)};
	let mut i = l.iter();

	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert!(i.next().is_none());
}

#[test]
fn test_iter_circular(){
	let l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,0)};
	let mut i = l.iter_circular();

	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');

	let l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,1)};
	let mut i = l.iter_circular();

	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');

	let l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,2)};
	let mut i = l.iter_circular();

	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
	assert_eq!(*i.next().unwrap(),'c');
	assert_eq!(*i.next().unwrap(),'a');
	assert_eq!(*i.next().unwrap(),'b');
}

#[test]
fn test_internal_index(){
	let mut l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,0)};

	l.first = 0;
	assert_eq!(l.internal_index(0),0);
	assert_eq!(l.internal_index(1),1);
	assert_eq!(l.internal_index(2),2);
	assert_eq!(l.internal_index(3),0);
	assert_eq!(l.internal_index(4),1);
	assert_eq!(l.internal_index(5),2);
	assert_eq!(l.internal_index(6),0);

	l.first = 1;
	assert_eq!(l.internal_index(0),1);
	assert_eq!(l.internal_index(1),2);
	assert_eq!(l.internal_index(2),0);
	assert_eq!(l.internal_index(3),1);
	assert_eq!(l.internal_index(4),2);
	assert_eq!(l.internal_index(5),0);
	assert_eq!(l.internal_index(6),1);

	l.first = 2;
	assert_eq!(l.internal_index(0),2);
	assert_eq!(l.internal_index(1),0);
	assert_eq!(l.internal_index(2),1);
	assert_eq!(l.internal_index(3),2);
	assert_eq!(l.internal_index(4),0);
	assert_eq!(l.internal_index(5),1);
	assert_eq!(l.internal_index(6),2);
}

#[test]
fn test_internal_index_reversed(){
	let mut l = unsafe{CircularBuffer::from_raw_parts(Box::new(['a','b','c']) as Box<[char]>,0)};

	l.first = 0;
	assert_eq!(l.internal_index_reversed(0),0);
	assert_eq!(l.internal_index_reversed(1),2);
	assert_eq!(l.internal_index_reversed(2),1);
	assert_eq!(l.internal_index_reversed(3),0);
	assert_eq!(l.internal_index_reversed(4),2);
	assert_eq!(l.internal_index_reversed(5),1);
	assert_eq!(l.internal_index_reversed(6),0);

	l.first = 1;
	assert_eq!(l.internal_index_reversed(0),1);
	assert_eq!(l.internal_index_reversed(1),0);
	assert_eq!(l.internal_index_reversed(2),2);
	assert_eq!(l.internal_index_reversed(3),1);
	assert_eq!(l.internal_index_reversed(4),0);
	assert_eq!(l.internal_index_reversed(5),2);
	assert_eq!(l.internal_index_reversed(6),1);

	l.first = 2;
	assert_eq!(l.internal_index_reversed(0),2);
	assert_eq!(l.internal_index_reversed(1),1);
	assert_eq!(l.internal_index_reversed(2),0);
	assert_eq!(l.internal_index_reversed(3),2);
	assert_eq!(l.internal_index_reversed(4),1);
	assert_eq!(l.internal_index_reversed(5),0);
	assert_eq!(l.internal_index_reversed(6),2);
}

#[test]
fn test_queue_reversed(){
	let mut l = CircularBuffer::from(Box::new(['a','b','c','d']) as Box<[char]>);
	assert_eq!(l.first,0);
	assert_eq!(&*l.list,&['a','b','c','d']);

	l.queue_reversed('9');
	assert_eq!(l.first,1);
	assert_eq!(&*l.list,&['9','b','c','d']);

	l.queue_reversed('8');
	assert_eq!(l.first,2);
	assert_eq!(&*l.list,&['9','8','c','d']);

	l.queue_reversed('7');
	assert_eq!(l.first,3);
	assert_eq!(&*l.list,&['9','8','7','d']);

	l.queue_reversed('6');
	assert_eq!(l.first,0);
	assert_eq!(&*l.list,&['9','8','7','6']);

	l.queue_reversed('5');
	assert_eq!(l.first,1);
	assert_eq!(&*l.list,&['5','8','7','6']);

	l.queue_reversed('4');
	assert_eq!(l.first,2);
	assert_eq!(&*l.list,&['5','4','7','6']);
}

#[test]
fn test_savedvalues1(){
	let mut l = iters::SavedValues::new(vec!['a','b','c','d','e','f','g','h','i'].into_iter(),4).unwrap();
	assert_eq!(&*l.0.list,&['d','c','b','a']);

	assert_eq!(l.next(),Some('e'));
	assert_eq!(&*l.0.list,&['d','c','b','e']);
	assert_eq!(*l.0.get(0),'e');
	assert_eq!(*l.0.get(1),'d');
	assert_eq!(*l.0.get(2),'c');
	assert_eq!(*l.0.get(3),'b');
	assert_eq!(*l.0.get(4),'e');
	assert_eq!(*l.0.get(5),'d');

	assert_eq!(l.next(),Some('f'));
	assert_eq!(&*l.0.list,&['d','c','f','e']);
	assert_eq!(*l.0.get(0),'f');
	assert_eq!(*l.0.get(1),'e');
	assert_eq!(*l.0.get(2),'d');
	assert_eq!(*l.0.get(3),'c');
	assert_eq!(*l.0.get(4),'f');
	assert_eq!(*l.0.get(5),'e');


	assert_eq!(l.next(),Some('g'));
	assert_eq!(&*l.0.list,&['d','g','f','e']);

	assert_eq!(l.next(),Some('h'));
	assert_eq!(&*l.0.list,&['h','g','f','e']);

	assert_eq!(l.next(),Some('i'));
	assert_eq!(&*l.0.list,&['h','g','f','i']);

	assert_eq!(l.next(),None);
	assert_eq!(&*l.0.list,&['h','g','f','i']);

	assert_eq!(l.next(),None);
	assert_eq!(&*l.0.list,&['h','g','f','i']);
}

#[test]
fn test_savedvalues2(){
	let mut l = iters::SavedValues::new(vec!['a','b','c'].into_iter(),1).unwrap();
	assert_eq!(&*l.0.list,&['a']);

	assert_eq!(l.next(),Some('b'));
	assert_eq!(&*l.0.list,&['b']);

	assert_eq!(l.next(),Some('c'));
	assert_eq!(&*l.0.list,&['c']);

	assert_eq!(l.next(),None);
	assert_eq!(&*l.0.list,&['c']);

	assert_eq!(l.next(),None);
	assert_eq!(&*l.0.list,&['c']);
}

#[test]
fn test_savedvalues3(){
	let mut l = iters::SavedValues::new(vec!['a','b','c'].into_iter(),3).unwrap();
	assert_eq!(&*l.0.list,&['c','b','a']);

	assert_eq!(l.next(),None);
	assert_eq!(&*l.0.list,&['c','b','a']);

	assert_eq!(l.next(),None);
	assert_eq!(&*l.0.list,&['c','b','a']);
}

#[test]
fn test_savedvalues4(){
	assert!(iters::SavedValues::new(vec!['a','b','c'].into_iter(),4).is_none());
}

#[test]
fn test_savedvalues5(){
	assert!(iters::SavedValues::new(vec!['a','b','c'].into_iter(),0).is_none());
}
