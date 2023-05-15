import init, { run } from './out/boids-quadtree.js'

init().then(()=>{
	run("#boids", 900, 900)
});
