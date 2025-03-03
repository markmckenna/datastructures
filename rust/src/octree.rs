/*! 
 * This datastructure, given a set of 3d points, subdivides the space across those points in such a way that it recursively splits
 * the space into "octants," equally sized sub-cubes of space each representing one-eigth of the volume enclosed by its parent element.
 * Each volume of space at each level of subdivision is represented by its centre point, which is the only point in common to all eight 
 * of its child cubes.
 * 
 * The octree is most effective at providing insight into a space of points from a top-down perspective. You can act on a set of points
 * at a level of approximation; and efficiently find nearest neighbours in each dimension. 
 * 
 * The octree is spatially subdivided, not content subdivided (in a content subdivided tree you'd expect basically the same number of points
 * in each child, but the volume of space for each child might be different; in a spatially subdivided tree, each child covers the same volume,
 * but could have radically different numbers of children).
 * 
 * Each point in the octree may have arbitrary data associated with it, and the structure can have its interior nodes augmented with additional
 * data that makes it useful for different kinds of query. For example, if you wanted to store shapes rather than points, you could have the structure
 * move shapes that cross sub-cubes up to an interior node that encompasses all of its vertices; or you could store the vertices in each leaf, but
 * decorate interior nodes with stats that help you locate all the vertices associated with a shape.
 */ 

/* Define a point structure for input. The simplest answer would be a packed integer format but I don't want this algorithm to be hard-coded like that.
 *
 * What's the most efficient way to make the point structure be changeable, but have the algo below still be able to unpack them?
 * - I think the easiest way to do this is probably with templates, but that would require stepping up to c++. Not the end of the world but if the API
 *   could be pure C that would be cooler.
 * - In C, I'd need the consumer to supply function pointers along with the data so that they could tell me how to unpack the indices. And I wouldn't
 *   know the traits of the data going into those functions so I wouldn't be able to adjust the behaviour the way I wanted.
 * Resolution: Templates are the way to go here.
 * 
 * The octree stores positions implicitly based on the cell's location within the tree. It requires the space to be cubical, and indexed by an integer
 * that uses a fixed bit depth which is the same in each axis (eg. 32bpp or 96b per point).
 * - Each level in the tree represents 1 bit in each plane, starting with the most significant.
 * - Each node at a level (indexed 0 through 7) represents one combination of bits at that level of significance.
 * - In effect, a node's depth determines the bit position in each plane, and the node's index determines the value of that bit for each plane.
 * - Each node may have a data pointer referencing point data in that region of space.
 * - The tree as a whole is 'unrolled' down to a certain level of resolution. Digits beyond that are stored in more traditional coordinate form.
 * 
 * For clarity, imagine a space with coordinates of only 3bits per plane. This tiny space only supports 512 distinct locations. 
 * - The entire, fully unrolled, octree would consist of 1 root node, 8 first-level children, 64 second-level children, and 512 leaf children.
 * - The 'path' from root to a specific leaf, read as a series of bits in each axis, rebuilds the index of that location in space.
 * 
 * This tree, in its most simple form, just provides a different way to look at storing point data. Imagine you had a whole bunch of data points indexed
 * using the 3bpp scheme above. If you allocate an array of 512 elements, where each element is a pointer to a list of vertex data, then you could 
 * simply interleave the bits from each plane of the input point, and the result would give you the index in the array where that point should be stored.
 * The resulting array effectively groups data points according to their location in space (in this example, only points located at the identical location
 * in space will be grouped together). In this scheme, you would no longer need to store the points themselves; just the data at those points (because each
 * cell describes a unique location in space).
 * 
 * Note the octree subdivides the space, not the points. If a location has no data points in it, it still takes up space in the tree. You most often want
 * the octree to stop 'early', indexing only the first k bits in each axis, or stopping when the average bucket capacity is at or below a specific value,
 * or something like that. This makes the octree into a 'bucketing' strategy, grouping together points in the same sector of space.
 * 
 * It's also possible to augment the tree so that interior nodes get data associated with them as well. These could be used for objects that occupy
 * multiple regions of space, brighter objects (ie visible from further away), lower-resolution copies of objects, and so on.
 * 
 * The tree also makes it relatively easy to find the nearest neighbours of a given point. Since each node represents a cube of space, it has 6
 * neighbours that it shares a face with, and 26 neighbours that it shares at least one vertex with. The index of each face-neighbour is +1 or -1 of this
 * node's address in one axis, while the index of each vertex-neighbour is +1 or -1 in any combination of axes. The nearest neighbour is highly likely to be
 * in one of those 26 cells (NOTE: not guaranteed! Distance is described by a sphere of a given radius, and this tree splits space into cubes, making it an
 * imperfect fit for this use case).
 * 
 * Because of the way the structure subdivides space it is very well suited for voxel-space applications, where data describes the state of a region of space;
 * rather than a space being filled using a collection of data points, such as in the more common vector-space models.
 * 
 * Applications:
 * - Level of Detail (LoD) rendering. Depth-based traversal of the tree can be curtailed at any time, so the exploration of the tree can be short-circuited
 *   based on the computed distance of the nearest corner of each cell to an observer. Interior nodes can provide lower-detail versions of objects.
 * - Colour compression. RGB (and HSV) colours are effectively 3d points as well, with an axis for each colour plane, so you can use this structure to
 *   render colours with lower precision as well.
 * - Optimized raycasting algorithms. Since you can find adjacent cells simply by doing math on the current cell's position, you can efficiently walk along
 *   a ray at a given angle using a Bresenham-like integer algorithm, finding all cells on that ray, in order.
 */

use alloc::Vec;

/// Indexing point type for octree
struct Point<T : i32>(T, T, T);

/// Data storage for leveled octree impl (number of levels is a compile-time constant)
struct Octree<T : i32, const Depth: u8> {
    data: Box<[Vec<Point>; 1<<(3*Depth)]>
}

impl Octree {
    type Point = Point<T>;
    type PointList = Vec<Self::Point>;

    /// Define a new fully populated octree, built over the given list of [points].
    fn new(points: PointList) -> Self {
        // loop through the points to fit them into the octree-oriented datastructure
    }
}

fn main() {
    let x = Octree::new([]);
}