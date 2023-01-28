(function() {var implementors = {
"object_detector_rust":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/bbox/struct.BBox.html\" title=\"struct object_detector_rust::bbox::BBox\">BBox</a>"],["impl&lt;X, Y, C1, C2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/classifier/struct.CombinedClassifier.html\" title=\"struct object_detector_rust::classifier::CombinedClassifier\">CombinedClassifier</a>&lt;X, Y, C1, C2&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;X: Float + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Y: Label + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C1: <a class=\"trait\" href=\"object_detector_rust/classifier/trait.Classifier.html\" title=\"trait object_detector_rust::classifier::Classifier\">Classifier</a>&lt;X, Y&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C2: <a class=\"trait\" href=\"object_detector_rust/classifier/trait.Classifier.html\" title=\"trait object_detector_rust::classifier::Classifier\">Classifier</a>&lt;X, Y&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"],["impl&lt;X, Y&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/classifier/struct.BayesClassifier.html\" title=\"struct object_detector_rust::classifier::BayesClassifier\">BayesClassifier</a>&lt;X, Y&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;X: Float + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Y: Label + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"],["impl&lt;X, Y&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/classifier/struct.SVMClassifier.html\" title=\"struct object_detector_rust::classifier::SVMClassifier\">SVMClassifier</a>&lt;X, Y&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;X: Float + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Y: Label + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"],["impl&lt;X, Y&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/classifier/struct.RandomForestClassifier.html\" title=\"struct object_detector_rust::classifier::RandomForestClassifier\">RandomForestClassifier</a>&lt;X, Y&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;X: Number + FloatNumber + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Y: Number + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/dataset/struct.MemoryDataSet.html\" title=\"struct object_detector_rust::dataset::MemoryDataSet\">MemoryDataSet</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/detection/struct.Detection.html\" title=\"struct object_detector_rust::detection::Detection\">Detection</a>"],["impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/detector/struct.BriefSVMDetector.html\" title=\"struct object_detector_rust::detector::BriefSVMDetector\">BriefSVMDetector</a>&lt;W&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"object_detector_rust/utils/trait.WindowGenerator.html\" title=\"trait object_detector_rust::utils::WindowGenerator\">WindowGenerator</a>&lt;DynamicImage&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"],["impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/detector/struct.HOGSVMDetector.html\" title=\"struct object_detector_rust::detector::HOGSVMDetector\">HOGSVMDetector</a>&lt;W&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"object_detector_rust/utils/trait.WindowGenerator.html\" title=\"trait object_detector_rust::utils::WindowGenerator\">WindowGenerator</a>&lt;DynamicImage&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/feature/struct.HOGFeature.html\" title=\"struct object_detector_rust::feature::HOGFeature\">HOGFeature</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/feature/struct.BriefFeature.html\" title=\"struct object_detector_rust::feature::BriefFeature\">BriefFeature</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/feature/struct.DummyFeature.html\" title=\"struct object_detector_rust::feature::DummyFeature\">DummyFeature</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/types/struct.Annotation.html\" title=\"struct object_detector_rust::types::Annotation\">Annotation</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/types/struct.AnnotatedImage.html\" title=\"struct object_detector_rust::types::AnnotatedImage\">AnnotatedImage</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/utils/struct.SlidingWindow.html\" title=\"struct object_detector_rust::utils::SlidingWindow\">SlidingWindow</a>"],["impl&lt;I:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object_detector_rust/utils/struct.ImageWindow.html\" title=\"struct object_detector_rust::utils::ImageWindow\">ImageWindow</a>&lt;I&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()