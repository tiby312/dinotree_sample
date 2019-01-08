

extern crate axgeom;
extern crate ordered_float;
extern crate dinotree;
extern crate dists;

use ordered_float::NotNan; 


use axgeom::Rect;

pub struct Bot{
	aabb:axgeom::Rect<NotNan<f32>>,
	vec:[f32;2]
}

impl Bot{
	pub fn collide(&mut self,bot:&mut Bot,_radius:f32){
		let a=&mut self.vec;
		let b=&mut bot.vec;

		let mut offsetx=b[0]-a[0];
		let mut offsety=b[1]-b[1];

		offsetx*=0.001;
		offsety*=0.001;

		a[0]+=offsetx;
		b[0]+=offsety;

		//TODO normalize
	}
	pub fn get_vec(&self)->[f32;2]{
		self.vec
	}
}

unsafe impl dinotree::HasAabb for Bot{
	type Num=NotNan<f32>;
	fn get(&self)->&axgeom::Rect<Self::Num>{
		&self.aabb
	}
}





pub struct SampleBuilder{
	grow:f64,
	radius:f32,
	num:usize,
}

impl SampleBuilder{
	pub fn new()->SampleBuilder{
		let grow=1.0;
		let radius=5.0;
		let num=10_000;
		SampleBuilder{num,grow,radius}
	}

	pub fn with_num(&mut self,num:usize){
		self.num=num;
	}

	pub fn with_radius_of(&mut self,radius:f32){
		self.radius=radius;
	}

	pub fn build(self)->Vec<Bot>{
		let dist=dists::spiral::Spiral::new([0.0,0.0],17.0,self.grow);
    	
		let radius=self.radius;
		dist.take(self.num).map(|a|Bot{aabb:unsafe{create_aabb([a[0] as f32,a[1] as f32],radius)},vec:[0.0;2]}).collect()
	}
}


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
	let bots=dinotree_alg::SampleBuilder::new().build();
	let tree=dinotree:DinoTreeNoCopyBuilder::new(axgeom::XAXISS,&mut bots);
	dinotree_alg::QueryBuilder::new(tree.as_ref_mut(),|a,b|{
		a.collide(b);
	});
}
*/


