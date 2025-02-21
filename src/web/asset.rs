use std::collections::HashMap;

use std::sync::LazyLock;

pub(crate) static ASSETS_MAP: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {
    HashMap::from([
        (r"/assets/inbound-bot-PJJg_rST.png", 0),
        (r"/assets/index-5cCgN66_.css", 1),
        (r"/assets/index-D-8kzAxq.js", 2),
        (r"/assets/outbound-bot-EmsLuWRN.png", 3),
        (r"/assets/text-bot-CWb_Poym.png", 4),
        (r"/assets/usedByDialogNodeTextGeneration-DrFqkTqi.png", 5),
        (
            r"/assets/usedByDialogNodeTextGeneration-thumbnail-C1iQCVQO.png",
            6,
        ),
        (r"/assets/usedByLlmChatNode-Bv2Fg5P7.png", 7),
        (r"/assets/usedBySentenceEmbedding-Dmju1hVB.png", 8),
        (r"/assets/usedBySentenceEmbedding-thumbnail-DVXz_sh0.png", 9),
        (r"/favicon.ico", 10),
        ("/", 11),
        (r"/index.html", 11),
    ])
});
