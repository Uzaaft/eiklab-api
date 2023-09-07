pub mod ner;
use ner::NERClassifier;

pub struct NER {
    pub classifier: NERClassifier,
}
