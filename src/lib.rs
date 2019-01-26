

extern crate axgeom;
extern crate ordered_float;
extern crate dists;

use ordered_float::NotNan; 


use axgeom::Rect;

#[derive(Copy,Clone)]
pub struct Bot{
	pub acc:usize,
	id:usize,
	pos:[f32;2]
}

impl Bot{
	pub fn collide(&mut self,bot:&mut Bot){
		self.acc.wrapping_add(bot.id);
		bot.acc.wrapping_add(self.id);
	}
	pub fn id(&self)->usize{
		self.id
	}
	fn create_aabb(&self,radius:f32)->axgeom::Rect<NotNan<f32>>{
		unsafe{create_aabb(self.pos,radius)}
	}
}

impl Iterator for SampleBuilderIter{
	type Item=Bot;
	fn next(&mut self)->Option<Self::Item>{
		self.spiral.next().map(|(id,pos)|{
			let bot=Bot{acc:0,id,pos:[pos[0] as f32,pos[1] as f32]};
			//let rect=bot.create_aabb(self.radius);
			bot
		})
	}
}
impl std::iter::FusedIterator for SampleBuilderIter{}

pub struct SampleBuilderIter{
	spiral:std::iter::Enumerate<dists::spiral::SpiralF32>,
	//radius:f32
}

pub struct SampleBuilder{
	grow:f64,
	radius:f32,
	num:usize,
}

impl SampleBuilder{
	pub fn create_aabb(&self,bot:&Bot)->axgeom::Rect<NotNan<f32>>{
		bot.create_aabb(self.radius)
	}
	pub fn new()->SampleBuilder{
		let grow=1.0;
		let radius=5.0;
		let num=1_000;
		SampleBuilder{num,grow,radius}
	}

	pub fn with_grow(&mut self,grow:f64)->&mut Self{
		self.grow=grow;
		self
	}
	pub fn with_num(&mut self,num:usize)->&mut Self{
		self.num=num;
		self
	}

	pub fn with_radius_of(&mut self,radius:f32)->&mut Self{
		self.radius=radius;
		self
	}
	pub fn build(&self)->SampleBuilderIter{
		let spiral=dists::spiral::Spiral::new([0.0,0.0],17.0,self.grow).as_f32();
    	let spiral=spiral.enumerate();
    	SampleBuilderIter{spiral}
	}
}
/*
fn is_valid<N:Ord+Copy>(a:&Rect<N>)->bool{
	a.0[0].left<=a.0[0].right && a.0[1].left<=a.0[1].right
}
*/


unsafe fn create_aabb(a:[f32;2],radius:f32)->Rect<NotNan<f32>>{
	from_rect_unchecked(aabb_from_pointf32(a,[radius;2]))

}


unsafe fn from_rect_unchecked(rect:Rect<f32>)->Rect<NotNan<f32>>{
    let ((a,b),(c,d))=rect.get();
    Rect::new(NotNan::unchecked_new(a),NotNan::unchecked_new(b),NotNan::unchecked_new(c),NotNan::unchecked_new(d))
}




fn aabb_from_pointf32(p:[f32;2],r:[f32;2])->Rect<f32>{
    Rect::new(p[0]-r[0],p[0]+r[0],p[1]-r[1],p[1]+r[1])
}


/*
fn test_basic(){
	use SampleBuilder;
	let builder=SampleBuilder::new();
	let bots=builder.build().collect();

	let tree=dinotree:DinoTreeBuilder::new(axgeom::XAXISS,&mut bots,|a|builder.create_aabb(a));
	dinotree_alg::QueryBuilder::new(tree.as_ref_mut(),|a,b|{
		a.collide(b);
	});
}
*/