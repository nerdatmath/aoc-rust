aoc::parts!(1, 2);

#[derive(Clone, Debug)]
enum Block {
    File(usize),
    Free,
}

#[derive(Clone, Debug)]
struct Span {
    pos: usize,
    size: usize,
    block: Block,
}

impl Span {
    fn is_free(&self) -> bool {
        match *self {
            Span {
                pos: _,
                size,
                block: Block::Free,
            } if size > 0 => true,
            _ => false,
        }
    }

    fn is_file(&self) -> bool {
        match *self {
            Span {
                pos: _,
                size: _,
                block: Block::File(_),
            } => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Disk(Vec<Span>);

impl Disk {
    fn files_iter(&self) -> impl Iterator<Item = &Span> {
        self.0.iter().rev().filter(|span| span.is_file())
    }

    fn checksum(&self) -> usize {
        return {
            self.0
                .iter()
                .map(|Span { pos, size, block }| match block {
                    Block::File(id) => size * (2 * pos + size - 1) / 2 * id,
                    Block::Free => 0,
                })
                .sum()
        };
    }
}

impl std::str::FromStr for Disk {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fileid = 0usize;
        let mut spans = Vec::<Span>::new();
        let mut file = true;
        let mut pos = 0usize;
        for ch in s.chars() {
            let st: String = ch.into();
            let size: usize = st.parse()?;
            let block = if file {
                let block = Block::File(fileid);
                fileid += 1;
                block
            } else {
                Block::Free
            };
            spans.push(Span { pos, size, block });
            pos += size;
            file = !file;
        }
        Ok(Disk(spans))
    }
}

trait Allocator {
    fn alloc(&mut self, size: &mut usize, max_pos: usize) -> Vec<Span>;
}

trait Part {
    fn allocator(disk: &Disk) -> impl Allocator;

    fn defrag(disk: &mut Disk) {
        let mut spans = vec![];
        {
            let mut freelist = Self::allocator(disk);
            for file in disk.files_iter() {
                let mut file = file.clone();
                let mut free_spans = freelist.alloc(&mut file.size, file.pos);
                for span in &mut free_spans {
                    span.block = file.block.clone();
                }
                spans.append(&mut free_spans);
                if file.size > 0 {
                    spans.push(file);
                }
            }
        }
        disk.0 = spans;
    }

    fn run(input: aoc::Input) -> usize {
        let mut disk: Disk = input.raw().parse().expect("Parsing error.");
        // dbg!(&disk);
        Self::defrag(&mut disk);
        // dbg!(&disk);
        disk.checksum()
    }
}

mod part_1 {
    pub enum Impl {}

    struct Allocator {
        spans: Vec<super::Span>,
    }

    impl Allocator {
        fn new(disk: &super::Disk) -> Self {
            Self {
                spans: disk
                    .0
                    .iter()
                    .filter(|span| span.is_free())
                    .rev()
                    .cloned()
                    .collect(),
            }
        }
    }

    impl super::Allocator for Allocator {
        fn alloc(&mut self, size: &mut usize, max_pos: usize) -> Vec<super::Span> {
            let mut spans = vec![];
            while *size > 0 {
                if let Some(mut free) = self.spans.pop() {
                    if free.pos >= max_pos {
                        break;
                    }
                    let mut span = free.clone();
                    if span.size > *size {
                        span.size = *size;
                    };
                    free.size -= span.size;
                    free.pos += span.size;
                    *size -= span.size;
                    spans.push(span);
                    if free.size > 0 {
                        self.spans.push(free);
                    }
                } else {
                    break;
                }
            }
            spans
        }
    }

    impl super::Part for Impl {
        fn allocator(disk: &super::Disk) -> impl super::Allocator {
            Allocator::new(disk)
        }
    }
}

mod part_2 {
    pub enum Impl {}

    struct Allocator {
        spans: Vec<super::Span>,
    }

    impl Allocator {
        fn new(disk: &super::Disk) -> Self {
            Self {
                spans: disk
                    .0
                    .iter()
                    .filter(|span| span.is_free())
                    .cloned()
                    .collect(),
            }
        }
    }

    impl super::Allocator for Allocator {
        fn alloc(&mut self, size: &mut usize, max_pos: usize) -> Vec<super::Span> {
            match self
                .spans
                .iter_mut()
                .take_while(|s| s.pos < max_pos)
                .find(|s| s.size >= *size)
            {
                Some(free) => {
                    let mut span = free.clone();
                    span.size = *size;
                    free.pos += span.size;
                    free.size -= span.size;
                    *size = 0;
                    vec![span]
                }
                None => vec![],
            }
        }
    }

    impl super::Part for Impl {
        fn allocator(disk: &super::Disk) -> impl super::Allocator {
            Allocator::new(disk)
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    part_1::Impl::run(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_2::Impl::run(input)
}
