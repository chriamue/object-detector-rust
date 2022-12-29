var searchIndex = JSON.parse('{\
"object_detector_rust":{"doc":"Object Detector Rust 🔍","t":[3,3,2,6,12,11,11,11,11,0,12,11,11,11,11,12,11,11,11,11,0,11,11,0,11,11,11,11,11,11,12,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,8,8,3,3,3,10,11,10,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,10,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,8,3,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,2,2,2,2,5],"n":["AnnotatedImage","Annotation","BBox","Class","annotations","approx_from","approx_from","approx_into","approx_into","bbox","bbox","borrow","borrow","borrow_mut","borrow_mut","class","clone","clone","clone_into","clone_into","dataset","eq","eq","feature","fmt","fmt","from","from","from_subset","from_subset","image","into","into","is_in_subset","is_in_subset","prelude","to_owned","to_owned","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","utils","value_from","value_from","value_into","value_into","vzip","vzip","BBox","approx_from","approx_into","borrow","borrow_mut","bottom","clone","clone_into","eq","fmt","from","from_subset","height","intersect_area","intersect_height","intersect_width","into","is_in_subset","left","new","overlap","right","self_area","to_owned","to_subset","to_subset_unchecked","top","try_from","try_from","try_into","try_into","type_id","union_area","value_from","value_into","vzip","width","x","y","AnnotatedImageSet","DataSet","DummyDataSet","FolderDataSet","MemoryDataSet","add_annotated_image","add_annotated_image","annotated_images","annotated_images","annotated_images_len","annotated_images_len","approx_from","approx_from","approx_from","approx_into","approx_into","approx_into","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","default","default","from","from","from","from_subset","from_subset","from_subset","get_data","get_data","get_data","get_data","into","into","into","is_in_subset","is_in_subset","is_in_subset","len","len","len","len","load","load","load","load","load_label_names","new","new","new","to_subset","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","value_from","value_from","value_from","value_into","value_into","value_into","vzip","vzip","vzip","DummyFeature","Feature","HOGFeature","approx_from","approx_from","approx_into","approx_into","borrow","borrow","borrow_mut","borrow_mut","default","extract","extract","extract","fmt","from","from","from_subset","from_subset","into","into","is_in_subset","is_in_subset","new","new","new_with_options","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","value_from","value_from","value_into","value_into","vzip","vzip","AnnotatedImage","Annotation","BBox","Class","crop_bbox"],"q":["object_detector_rust","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","object_detector_rust::bbox","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","object_detector_rust::dataset","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","object_detector_rust::feature","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","object_detector_rust::prelude","","","","object_detector_rust::utils"],"d":["Image annotated by list of Annotations","annotation is a object bounding box in image and class type","","object class type","Annotations for the image","","","","","This module contains the <code>BBox</code> struct for representing …","Bounding box of the object","","","","","Class of the object","","","","","This module contains the <code>DataSet</code> trait for representing …","","","This module contains traits and structs for feature …","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Image","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","the object-detector-rust prelude","","","","","","","","","","","","","","","","","Utility functions for working with images","","","","","","","Struct for representing a bounding box","","","","","Returns the bottom coordinate of the bounding box","","","","","Returns the argument unchanged.","","Height of the bounding box","Calculates the area of the intersection between two …","Calculates the height of the intersection between two …","Calculates the width of the intersection between two …","Calls <code>U::from(self)</code>.","","Returns the left coordinate of the bounding box","Creates a new bounding box with the specified coordinates …","Calculates the overlap between two bounding boxes as the …","Returns the right coordinate of the bounding box","Calculates the area of the bounding box","","","","Returns the top coordinate of the bounding box","","","","","","Calculates the area of the union between two bounding boxes","","","","Width of the bounding box","X coordinate of the top-left corner of the bounding box","Y coordinate of the top-left corner of the bounding box","trait of a set of annotated images","Trait representing a dataset of images and labels for …","Dummy dataset that returns a fixed set of images and labels","Struct for representing a dataset loaded from a folder","Struct for representing a dataset in memory","adds an annotated image","Adds an annotated image to the dataset","returns iterator over annotated images","returns iterator over images","returns length of annotated images","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Returns the images and labels for training","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Returns the number of images in the dataset","","","","Loads the dataset","","","","Loads a list of label names from the specified file path.","Creates a new dataset from the specified folder","Creates a new, empty dataset","Creates a new <code>DummyDataSet</code>","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Dummy feature descriptor that always returns the same value","Trait for feature descriptors that can be extracted from …","Struct for extracting HOG features from images.","","","","","","","","","Creates a new HOG feature extractor with default options","Extracts the feature descriptor from the given image","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Creates a new dummy feature descriptor with the given value","Creates a new HOG feature extractor","Creates a new <code>HogFeature</code> with the given <code>HogOptions</code>.","","","","","","","","","","","","","","","","","","","","","","","","","Crops an image to the dimensions specified in the bounding …"],"i":[0,0,0,0,3,2,3,2,3,0,2,2,3,2,3,2,2,3,2,3,0,2,3,0,2,3,2,3,2,3,3,2,3,2,3,0,2,3,2,3,2,3,2,2,3,3,2,2,3,3,2,3,0,2,3,2,3,2,3,0,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,0,0,0,0,0,27,13,27,13,27,13,18,13,17,18,13,17,18,13,17,18,13,17,13,17,18,13,17,18,13,17,28,18,13,17,18,13,17,18,13,17,28,18,13,17,28,18,13,17,18,18,13,17,18,13,17,18,13,17,18,18,13,13,17,17,18,18,13,13,17,17,18,13,17,18,13,17,18,13,17,18,13,17,0,0,0,25,23,25,23,25,23,25,23,23,29,25,23,23,25,23,25,23,25,23,25,23,25,23,23,25,23,25,23,25,25,23,23,25,25,23,23,25,23,25,23,25,23,25,23,0,0,0,0,0],"f":[0,0,0,0,0,[[],1],[[],1],[[],1],[[],1],0,0,[[]],[[]],[[]],[[]],0,[2,2],[3,3],[[]],[[]],0,[[2,2],4],[[3,3],4],0,[[2,5],6],[[3,5],6],[[]],[[]],[[]],[[]],0,[[]],[[]],[[],4],[[],4],0,[[]],[[]],[[],7],[[],7],[[]],[[]],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],8],[[],8],0,[[],1],[[],1],[[],1],[[],1],[[]],[[]],0,[[],1],[[],1],[[]],[[]],[9,10],[9,9],[[]],[[9,9],4],[[9,5],6],[[]],[[]],0,[[9,9],11],[[9,9],11],[[9,9],11],[[]],[[],4],[9,10],[[10,10,11,11],9],[[9,9],12],[9,10],[9,11],[[]],[[],7],[[]],[9,10],[[],1],[[],1],[[],1],[[],1],[[],8],[[9,9],11],[[],1],[[],1],[[]],0,0,0,0,0,0,0,0,[3],[[13,3]],[[],[[15,[14]]]],[13,[[15,[14]]]],[[],16],[13,16],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[]],[[]],[[]],[[]],[[]],[[]],[[],13],[[],17],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[18],[13],[17],[[]],[[]],[[]],[[],4],[[],4],[[],4],[[],16],[18,16],[13,16],[17,16],[[],[[1,[[15,[19]]]]]],[18,[[1,[[15,[19]]]]]],[13,[[1,[[15,[19]]]]]],[17,[[1,[[15,[19]]]]]],[20,[[22,[21]]]],[[20,11,11,[22,[21]]],18],[[],13],[[],17],[[],7],[[],7],[[],7],[[]],[[]],[[]],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],8],[[],8],[[],8],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[]],[[]],[[]],0,0,0,[[],1],[[],1],[[],1],[[],1],[[]],[[]],[[]],[[]],[[],23],[24,[[1,[[22,[12]],21]]]],[[25,24],[[1,[[22,[12]],21]]]],[[23,24],[[1,[[22,[12]],21]]]],[[23,5],6],[[]],[[]],[[]],[[]],[[]],[[]],[[],4],[[],4],[12,25],[[],23],[26,23],[[],7],[[],7],[[]],[[]],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],8],[[],8],[[],1],[[],1],[[],1],[[],1],[[]],[[]],0,0,0,0,[[24,9],24]],"p":[[4,"Result"],[3,"Annotation"],[3,"AnnotatedImage"],[15,"bool"],[3,"Formatter"],[6,"Result"],[4,"Option"],[3,"TypeId"],[3,"BBox"],[15,"i32"],[15,"u32"],[15,"f32"],[3,"MemoryDataSet"],[8,"Iterator"],[3,"Box"],[15,"usize"],[3,"DummyDataSet"],[3,"FolderDataSet"],[8,"Error"],[15,"str"],[3,"String"],[3,"Vec"],[3,"HOGFeature"],[4,"DynamicImage"],[3,"DummyFeature"],[3,"HogOptions"],[8,"AnnotatedImageSet"],[8,"DataSet"],[8,"Feature"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};