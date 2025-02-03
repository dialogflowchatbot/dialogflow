<script setup>
import { ref, reactive, onMounted, onUnmounted, provide } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { copyProperties, httpReq, getRobotType } from '../../assets/tools.js'
// import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n'
import chatPicThumbnail from '@/assets/usedByLlmChatNode-thumbnail.png'
import chatPic from '@/assets/usedByLlmChatNode.png'
import textGenerationPicThumbnail from '@/assets/usedByDialogNodeTextGeneration-thumbnail.png'
import textGenerationPic from '@/assets/usedByDialogNodeTextGeneration.png'
import sentenceEmbeddingPicThumbnail from '@/assets/usedBySentenceEmbedding-thumbnail.png'
import sentenceEmbeddingPic from '@/assets/usedBySentenceEmbedding.png'

const { t, tm } = useI18n();
const route = useRoute()
const router = useRouter();
const robotId = route.params.robotId
const robotType = getRobotType(robotId)
const maxSessionIdleMin = ref(30)

const goBack = () => {
    router.push({ name: 'robotDetail', params: { robotId: robotId } });
}

const similarityThreshold = ref(85)
const defaultEmailVerificationRegex = '[-\\w\\.\\+]{1,100}@[A-Za-z0-9]{1,30}[A-Za-z\\.]{2,30}';
const settings = reactive({
    version: 1017000,
    maxSessionIdleSec: 1800,
    smtpHost: '',
    smtpUsername: '',
    smtpPassword: '',
    smtpTimeoutSec: 60,
    emailVerificationRegex: '',
    chatProvider: {
        provider: {
            id: '',
            model: '',
        },
        apiUrl: '',
        apiUrlDisabled: false,
        showApiKeyInput: true,
        apiKey: '',
        max_token_len: 1000,
        connectTimeoutMillis: 5000,
        readTimeoutMillis: 10000,
        maxResponseTokenLength: 5000,
        proxyUrl: '',
    },
    textGenerationProvider: {
        provider: {
            id: '',
            model: '',
        },
        apiUrl: '',
        apiUrlDisabled: false,
        showApiKeyInput: true,
        apiKey: '',
        max_token_len: 1000,
        connectTimeoutMillis: 5000,
        readTimeoutMillis: 10000,
        maxResponseTokenLength: 5000,
        proxyUrl: '',
    },
    sentenceEmbeddingProvider: {
        provider: {
            id: '',
            model: '',
        },
        similarityThreshold: 0.85,
        apiUrl: '',
        apiUrlDisabled: false,
        showApiKeyInput: true,
        apiKey: '',
        connectTimeoutMillis: 5000,
        readTimeoutMillis: 10000,
        proxyUrl: '',
    },
    asrProvider: {
        enabled: false,
        provider: {
            id: '',
            model: '',
        },
        apiUrl: '',
        apiUrlDisabled: false,
        showApiKeyInput: true,
        apiKey: '',
        connectTimeoutMillis: 5000,
        readTimeoutMillis: 10000,
        proxyUrl: '',
    },
    ttsProvider: {
        enabled: false,
        provider: {
            id: '',
            model: '',
        },
        apiUrl: '',
        apiUrlDisabled: false,
        showApiKeyInput: true,
        apiKey: '',
        connectTimeoutMillis: 5000,
        readTimeoutMillis: 10000,
        proxyUrl: '',
    },
});
const formLabelWidth = '150px'
const loading = ref(false)
const smtpPassed = ref(false)
const smtpFailed = ref(false)
const smtpFailedDetail = ref('')
const showHfIncorrectChatModelTip = ref(false)
const showHfChatModelDownloadProgress = ref(false)
const chatModelRepository = ref('')
const showHfIncorrectGenerationModelTip = ref(false)
const showHfGenerationModelDownloadProgress = ref(false)
const textGenerationModelRepository = ref('')
const showHfIncorrectEmbeddingModelTip = ref(false)
const showHfEmbeddingModelDownloadProgress = ref(false)
const sentenceEmbeddingModelRepository = ref('')
const originalSentenceEmbeddingModelId = ref('')
const downloadingUrl = ref('')
const downloadingProgress = ref('')

onMounted(async () => {
    const t = await httpReq("GET", 'management/settings', { robotId: robotId }, null, null)
    console.log(t);
    if (t.status == 200) {
        copyProperties(t.data, settings);
        maxSessionIdleMin.value = settings.maxSessionIdleSec / 60;
        // const d = t.data;
        // settings.port = d.port;
        // settings.maxSessionDurationMin = d.maxSessionDurationMin;
        originalSentenceEmbeddingModelId.value = t.data.sentenceEmbeddingProvider.provider.id;
        console.log('originalSentenceEmbeddingModelId=', originalSentenceEmbeddingModelId)
        changeChatProvider(settings.chatProvider.provider.id);
        changeTextGenerationProvider(settings.textGenerationProvider.provider.id);
        changeSentenceEmbeddingProvider(settings.sentenceEmbeddingProvider.provider.id);
        changeTtsProvider(settings.ttsProvider.provider.id);
    }
    await checkHfModelFiles();
})
onUnmounted(
    () => {
        if (timeoutID != null)
            clearTimeout(timeoutID);
    }
);

async function checkHfModelFiles() {
    const repostories = new Map()
    if (settings.chatProvider.provider.id == 'HuggingFace') {
        for (let i = 0; i < chatModelOptions.length; i++) {
            console.log(chatModelOptions[i].value)
            if (chatModelOptions[i].value == settings.chatProvider.provider.model) {
                let l = chatModelOptions[i].value;
                const p = l.lastIndexOf(' ');
                if (p > -1)
                    l = l.substring(0, p);
                chatModelRepository.value = l;
                repostories.set(showHfIncorrectGenerationModelTip, l)
                break;
            }
        }
    } else showHfIncorrectChatModelTip.value = false;
    if (settings.textGenerationProvider.provider.id == 'HuggingFace') {
        for (let i = 0; i < textGenerationModelOptions.length; i++) {
            console.log(textGenerationModelOptions[i].value)
            if (textGenerationModelOptions[i].value == settings.textGenerationProvider.provider.model) {
                let l = textGenerationModelOptions[i].value;
                const p = l.lastIndexOf(' ');
                if (p > -1)
                    l = l.substring(0, p);
                textGenerationModelRepository.value = l;
                repostories.set(showHfIncorrectGenerationModelTip, l)
                break;
            }
        }
    } else showHfIncorrectGenerationModelTip.value = false;
    if (settings.sentenceEmbeddingProvider.provider.id == 'HuggingFace') {
        for (let i = 0; i < sentenceEmbeddingModelOptions.length; i++) {
            console.log(sentenceEmbeddingModelOptions[i].value)
            if (sentenceEmbeddingModelOptions[i].value == settings.sentenceEmbeddingProvider.provider.model) {
                let l = sentenceEmbeddingModelOptions[i].value;
                const p = l.lastIndexOf(' ');
                if (p > -1)
                    l = l.substring(0, p);
                sentenceEmbeddingModelRepository.value = l;
                repostories.set(showHfIncorrectEmbeddingModelTip, l)
                break;
            }
        }
    } else showHfIncorrectEmbeddingModelTip.value = false;
    if (repostories.size > 0) {
        const r = await httpReq("POST", 'management/settings/model/check/files', null, null, Array.from(repostories.values()));
        console.log(r);
        if (r && r.data) {
            for (let [k, v] of repostories.entries()) {
                if (r.data[v] == false) {
                    k.value = true;
                } else
                    k.value = false;
            }
        }
    }
    // showHfIncorrectGenerationModelTip.value = false;
    // showHfIncorrectEmbeddingModelTip.value = false;
    // if (settings.textGenerationProvider.provider.id != 'HuggingFace' && settings.sentenceEmbeddingProvider.provider.id != 'HuggingFace') {
    //     return;
    // }
    // const r = await httpReq("POST", 'management/settings/model/check', { robotId: robotId }, null, repostories.keys());
    // console.log(r);
    // if (r && r.status != 200) {
    //     for (let i = 0; i < sentenceEmbeddingModelOptions.length; i++) {
    //         console.log(sentenceEmbeddingModelOptions[i].value)
    //         if (sentenceEmbeddingModelOptions[i].value == settings.sentenceEmbeddingProvider.provider.model) {
    //             let l = sentenceEmbeddingModelOptions[i].label;
    //             const p = l.lastIndexOf(' ');
    //             if (p > -1)
    //                 l = l.substring(0, p);
    //             sentenceEmbeddingModelRepository.value = l;
    //             break;
    //         }
    //     }
    //     showHfIncorrectEmbeddingModelTip.value = true;
    //     // ElMessage.error(r.err.message);
    // } else
    //     showHfIncorrectEmbeddingModelTip.value = false;
}

async function save() {
    if (originalSentenceEmbeddingModelId.value != settings.sentenceEmbeddingProvider.provider.id) {
        ElMessageBox.confirm(
            'Sentence embedding model has been changed, this may cause dimension mismatch issue <strong>(You can regenerate all sentences to resolve)</strong>. Continue?',
            'Warning',
            {
                confirmButtonText: 'OK',
                cancelButtonText: 'Cancel',
                type: 'warning',
                dangerouslyUseHTMLString: true,
            }
        )
            .then(() => {
                saveSettings()
            })
            .catch(() => {
            })
    } else saveSettings();
}

async function saveSettings() {
    if (!settings.emailVerificationRegex)
        settings.emailVerificationRegex = defaultEmailVerificationRegex;
    settings.maxSessionIdleSec = (maxSessionIdleMin.value * 60)
    settings.sentenceEmbeddingProvider.similarityThreshold = similarityThreshold.value / 100;
    let r = await httpReq("POST", 'management/settings', { robotId: robotId }, null, settings)
    console.log(r);
    if (r.status == 200) {
        ElMessage({ type: 'success', message: t('lang.common.saved'), });
        await checkHfModelFiles();
    } else {
        const m = t(r.err.message);
        ElMessage.error(m ? m : r.err.message);
    }
}

let timeoutID = null;

async function downloadModels(m) {
    const r = await httpReq("GET", 'management/settings/model/download/progress', null, null, null);
    console.log(r);
    if (r != null && r.data != null && r.data.downloading) {
        const m = 'Downloading: ' + r.url + ' (' + (r.data.downloadedLen / r.data.totalLen * 100).toFixed(2) + '), please wait until it finish.';
        ElMessage.error(m);
        return;
    }
    httpReq("POST", 'management/settings/model/download', { robotId: robotId, m: m }, null, m).then((r) => {
        console.log(r);
        if (r == null || r.status != 200) {
            ElMessage.error('Download failed: ' + r.err.message);
            return;
        }
        // showHfEmbeddingModelDownloadProgress.value = false;
        // showHfIncorrectEmbeddingModelTip.value = true;
        if (m == 'sentenceEmbedding') {
            showHfIncorrectEmbeddingModelTip.value = false;
            showHfEmbeddingModelDownloadProgress.value = true;
        } else {
            showHfIncorrectGenerationModelTip.value = false;
            showHfGenerationModelDownloadProgress.value = true;
        }
        timeoutID = setTimeout(async () => {
            await showDownloadProgress();
        }, 1000);
    });
}

function downloadComplete() {
    clearTimeout(timeoutID);
    showHfIncorrectGenerationModelTip.value = false;
    showHfGenerationModelDownloadProgress.value = false;
    showHfIncorrectEmbeddingModelTip.value = false;
    showHfEmbeddingModelDownloadProgress.value = false;
}

async function showDownloadProgress() {
    const r = await httpReq("GET", 'management/settings/model/download/progress', null, null, null);
    console.log(r);
    if (r != null && r.data != null) {
        if (r.data.err) {
            ElMessage.error(r.data.err);
            clearTimeout(timeoutID);
            showHfGenerationModelDownloadProgress.value = false;
            showHfEmbeddingModelDownloadProgress.value = false;
            return
        } else if (r.data.downloading) {
            downloadingUrl.value = r.data.url;
            downloadingProgress.value = (r.data.downloadedLen / r.data.totalLen * 100).toFixed(2);
            timeoutID = setTimeout(async () => {
                await showDownloadProgress();
            }, 1000);
        } else
            downloadComplete()
    } else {
        downloadComplete();
    }
}

const smtpTest = async () => {
    loading.value = true
    const r = await httpReq("POST", 'management/settings/smtp/test', null, null, settings)
    console.log(r);
    if (r.status == 200) {
        smtpPassed.value = true
        smtpFailed.value = false
    } else {
        smtpFailedDetail.value = t(r.err.message);
        smtpPassed.value = false
        smtpFailed.value = true
    }
    loading.value = false
}

const ollamaModels = [
    { label: 'Meta Llama 3.1 8b', value: 'llama3.1:8b' },
    { label: 'Meta Llama 3.1 70b', value: 'llama3.1:70b' },
    { label: 'Meta Llama 3 8b', value: 'llama3:8b' },
    { label: 'Meta Llama 3 70b', value: 'llama3:70b' },
    { label: 'Phi-3 3.8b', value: 'phi3:3.8b' },
    { label: 'Phi-3 14b', value: 'phi3:14b' },
    { label: 'Phi-3 instruct', value: 'phi3:instruct' },
    { label: 'Gemma2 9b', value: 'gemma2:9b' },
    { label: 'Gemma2 27b', value: 'gemma2:27b' },
    { label: 'WizardLM-2 7b', value: 'wizardlm2:7b' },
    { label: 'WizardLM-2 8x22b', value: 'wizardlm2:8x22b' },
    { label: 'Mistral 7b', value: 'mistral:7b' },
    { label: 'Mixtral 8x7b', value: 'mixtral:8x7b' },
    { label: 'Mixtral 8x22b', value: 'mixtral:8x22b' },
    { label: 'Qwen 2 1.5b', value: 'qwen2:1.5b' },
    { label: 'Qwen 2 7b', value: 'qwen2:7b' },
    { label: 'Qwen 2 72b', value: 'qwen2:72b' },
    { label: 'TinyLlama 1.1b', value: 'tinyllama:1.1b' },
    { label: 'Yi 1.5 6b', value: 'yi:6b' },
    { label: 'Yi 1.5 9b', value: 'yi:9b' },
    { label: 'Yi 1.5 34b', value: 'yi:34b' },
]
provide('ollamaModels', { ollamaModels })

// https://docs.spring.io/spring-ai/reference/api/embeddings.html
const chatProviders = [
    {
        id: 'HuggingFace',
        name: 'HuggingFace',
        apiUrl: 'Model will be downloaded locally at ./data/models',
        apiUrlDisabled: true,
        showApiKeyInput: false,
        models: [
            { label: 'microsoft/Phi-3-mini-4k-instruct (7.7GB)', value: 'Phi3Mini4kInstruct' },
            { label: 'microsoft/Phi-3-mini-128k-instruct (7.7GB)', value: 'Phi3Mini128kInstruct' },
            { label: 'microsoft/Phi-3-small-8k-instruct (15GB)', value: 'Phi3Small8kInstruct' },
            { label: 'microsoft/Phi-3-small-128k-instruct (15GB)', value: 'Phi3Small128kInstruct' },
            { label: 'microsoft/Phi-3-medium-4k-instruct (30GB)', value: 'Phi3Medium4kInstruct' },
            { label: 'microsoft/Phi-3-medium-128k-instruct (30GB)', value: 'Phi3Medium128kInstruct' },
            { label: 'google/gemma-2b-it (4.9GB)', value: 'Gemma2bInstruct', need_auth_header: true },
            { label: 'google/gemma-7b-it (12.1GB)', value: 'Gemma7bInstruct', need_auth_header: true },
            { label: 'meta-llama/Meta-Llama-3-8B-Instruct (??GB)', value: 'MetaLlama3_8bInstruct', need_auth_header: true },
            { label: 'upstage/SOLAR-10.7B-v1.0 (21.5GB)', value: 'Solar10_7bV1_0' },
            { label: 'Qwen/Qwen2-7B-Instruct (15.4GB)', value: 'Qwen2_72BInstruct', dimenssions: 384 },
            { label: 'Qwen/Qwen2-72B-Instruct (144GB)', value: 'Qwen2_72BInstruct', dimenssions: 384 },
            { label: 'TinyLlama/TinyLlama-1.1B-Chat-v1.0 (2.2GB)', value: 'TinyLlama1_1bChatV1_0' },
        ]
    },
    {
        id: 'OpenAI',
        name: 'OpenAI',
        apiUrl: 'https://api.openai.com/v1/chat/completions',
        apiUrlDisabled: true,
        showApiKeyInput: true,
        models: [
            { label: 'gpt-4o', value: 'gpt-4' },
            { label: 'gpt-4o-mini', value: 'gpt-4-mini' },
            { label: 'gpt-4', value: 'gpt-4' },
            { label: 'gpt-4-turbo', value: 'gpt-4-turbo' },
            { label: 'gpt-4-vision-preview', value: 'gpt-4-vision-preview' },
            { label: 'gpt-4-32k', value: 'gpt-4-32k' },
            { label: 'gpt-3.5-turbo', value: 'gpt-3.5-turbo' },
            { label: 'gpt-3.5-turbo-16k', value: 'gpt-3.5-turbo-16k' },
            { label: 'gpt-3.5-turbo', value: 'gpt-3.5-turbo' },
        ]
    },
    {
        id: 'Ollama',
        name: 'Ollama',
        apiUrl: 'http://localhost:11434/api/chat',
        apiUrlDisabled: false,
        showApiKeyInput: false,
        models: ollamaModels,
    },
]

const chatProviderProxyEnabled = ref(false)
const isAddingAnotherChatOllamaModel = ref(false)
const anotherChatOllamaModel = ref('')
const chatModelSelector = ref()
const addAnotherChatOllamaModel = (m) => {
    const obj = { label: m, value: m };
    chatModelOptions.unshift(obj);
    chatProviders[2].models.unshift(obj);
    chatModelSelector.value.blur();
    settings.chatProvider.provider.id = 'Ollama';
    settings.chatProvider.provider.model = obj.value;
    anotherChatOllamaModel.value = '';
}

const textGenerationProviders = [
    {
        id: 'HuggingFace',
        name: 'HuggingFace',
        apiUrl: 'Model will be downloaded locally at ./data/models',
        apiUrlDisabled: true,
        showApiKeyInput: false,
        models: [
            { label: 'microsoft/Phi-3-mini-4k-instruct (7.7GB)', value: 'Phi3Mini4kInstruct' },
            { label: 'microsoft/Phi-3-mini-128k-instruct (7.7GB)', value: 'Phi3Mini128kInstruct' },
            { label: 'microsoft/Phi-3-small-8k-instruct (15GB)', value: 'Phi3Small8kInstruct' },
            { label: 'microsoft/Phi-3-small-128k-instruct (15GB)', value: 'Phi3Small128kInstruct' },
            { label: 'microsoft/Phi-3-medium-4k-instruct (30GB)', value: 'Phi3Medium4kInstruct' },
            { label: 'microsoft/Phi-3-medium-128k-instruct (30GB)', value: 'Phi3Medium128kInstruct' },
            { label: 'google/gemma-2b-it (4.9GB)', value: 'Gemma2bInstruct', need_auth_header: true },
            { label: 'google/gemma-7b-it (12.1GB)', value: 'Gemma7bInstruct', need_auth_header: true },
            { label: 'meta-llama/Meta-Llama-3-8B-Instruct (??GB)', value: 'MetaLlama3_8bInstruct', need_auth_header: true },
            { label: 'upstage/SOLAR-10.7B-v1.0 (21.5GB)', value: 'Solar10_7bV1_0' },
            { label: 'Qwen/Qwen2-7B-Instruct (15.4GB)', value: 'Qwen2_72BInstruct', dimenssions: 384 },
            { label: 'Qwen/Qwen2-72B-Instruct (144GB)', value: 'Qwen2_72BInstruct', dimenssions: 384 },
            { label: 'TinyLlama/TinyLlama-1.1B-Chat-v1.0 (2.2GB)', value: 'TinyLlama1_1bChatV1_0' },
        ]
    },
    {
        id: 'OpenAI',
        name: 'OpenAI',
        apiUrl: 'https://api.openai.com/v1/chat/completions',
        apiUrlDisabled: true,
        showApiKeyInput: true,
        models: [
            { label: 'gpt-4', value: 'gpt-4' },
            { label: 'gpt-4-turbo-preview', value: 'gpt-4-turbo-preview' },
            { label: 'gpt-4-vision-preview', value: 'gpt-4-vision-preview' },
            { label: 'gpt-4-32k', value: 'gpt-4-32k' },
            { label: 'gpt-3.5-turbo', value: 'gpt-3.5-turbo' },
            { label: 'gpt-3.5-turbo-16k', value: 'gpt-3.5-turbo-16k' },
            { label: 'gpt-3.5-turbo', value: 'gpt-3.5-turbo' },
        ]
    },
    {
        id: 'Ollama',
        name: 'Ollama',
        apiUrl: 'http://localhost:11434/api/generate',
        apiUrlDisabled: false,
        showApiKeyInput: false,
        models: ollamaModels,
    },
]

const textGenerationProviderProxyEnabled = ref(false)
const isAddingAnotherTextGenerationOllamaModel = ref(false)
const anotherTextGenerationOllamaModel = ref('')
const textGenerationModelSelector = ref()
const addAnotherTextGenerationOllamaModel = (m) => {
    const obj = { label: m, value: m };
    textGenerationModelOptions.unshift(obj);
    textGenerationProviders[2].models.unshift(obj);
    textGenerationModelSelector.value.blur();
    settings.textGenerationProvider.provider.id = 'Ollama';
    settings.textGenerationProvider.provider.model = obj.value;
    anotherTextGenerationOllamaModel.value = '';
}

// https://docs.spring.io/spring-ai/reference/api/embeddings.html
const sentenceEmbeddingProviders = [
    {
        id: 'HuggingFace',
        name: 'HuggingFace',
        apiUrl: 'Model will be downloaded locally at ./data/models',
        apiUrlDisabled: true,
        showApiKeyInput: false,
        models: [
            { label: 'sentence-transformers/all-MiniLM-L6-v2 (91MB)', value: 'AllMiniLML6V2', dimenssions: 384 },
            { label: 'sentence-transformers/paraphrase-MiniLM-L12-v2 (135MB)', value: 'ParaphraseMLMiniLML12V2' },
            { label: 'sentence-transformers/paraphrase-multilingual-mpnet-base-v2 (1.11GB)', value: 'ParaphraseMLMpnetBaseV2' },
            { label: 'BAAI/bge-small-en-v1.5 (135MB)', value: 'BgeSmallEnV1_5' },
            { label: 'BAAI/bge-base-en-v1.5 (439MB)', value: 'BgeBaseEnV1_5' },
            { label: 'BAAI/bge-large-en-v1.5 (1.35GB)', value: 'BgeLargeEnV1_5' },
            { label: 'BAAI/bge-m3 (2.27GB)', value: 'BgeM3' },
            { label: 'nomic-ai/nomic-embed-text-v1.5 (550MB)', value: 'NomicEmbedTextV1_5' },
            { label: 'intfloat/multilingual-e5-small (472MB)', value: 'MultilingualE5Small' },
            { label: 'intfloat/multilingual-e5-base (1.11GB)', value: 'MultilingualE5Base' },
            { label: 'intfloat/multilingual-e5-large (2.24GB)', value: 'MultilingualE5Large' },
            { label: 'mixedbread-ai/mxbai-embed-large-v1 (1.34GB)', value: 'MxbaiEmbedLargeV1' },
            { label: 'moka-ai/m3e-base (409MB)', value: 'MokaAiM3eBase' },
            { label: 'moka-ai/m3e-large (1.3GB)', value: 'MokaAiM3eLarge' },
        ]
    },
    {
        id: 'OpenAI',
        name: 'OpenAI',
        apiUrl: 'https://api.openai.com/v1/embeddings',
        apiUrlDisabled: true,
        showApiKeyInput: true,
        models: [
            { label: 'text-embedding-3-large', value: 'text-embedding-3-large' },
            { label: 'text-embedding-3-small', value: 'text-embedding-3-small' },
            { label: 'text-embedding-ada-002', value: 'text-embedding-ada-002' }]
    },
    {
        id: 'Ollama',
        name: 'Ollama',
        apiUrl: 'http://localhost:11434/api/embeddings',
        apiUrlDisabled: false,
        showApiKeyInput: false,
        models: [
            { label: 'nomic-embed-text:v1.5', value: 'nomic-embed-text:v1.5' },
            { label: 'mxbai-embed-large:335m', value: 'mxbai-embed-large:335m' },
            { label: 'snowflake-arctic-embed:335m', value: 'snowflake-arctic-embed:335m' },
            { label: 'jina-embeddings-v2-base-en', value: 'jina/jina-embeddings-v2-base-en:latest' },
        ],
    },
]
const sentenceEmbeddingProviderProxyEnabled = ref(false)
const chatModelOptions = reactive([])
const chatDynamicReqUrlMap = new Map();
const choosedChatProvider = ref('')
const changeChatProvider = (n) => {
    if (choosedChatProvider.value)
        chatDynamicReqUrlMap.set(choosedChatProvider.value, settings.chatProvider.apiUrl);
    for (let i = 0; i < chatProviders.length; i++) {
        if (chatProviders[i].id == n) {
            if (chatProviders[i].apiUrlDisabled)
                settings.chatProvider.apiUrl = chatProviders[i].apiUrl;
            else {
                settings.chatProvider.apiUrl = sentenceEmbeddingDynamicReqUrlMap.get(settings.chatProvider.provider.id);
                if (!settings.chatProvider.apiUrl)
                    settings.chatProvider.apiUrl = chatProviders[i].apiUrl;
            }
            settings.chatProvider.apiUrlDisabled = chatProviders[i].apiUrlDisabled;
            settings.chatProvider.showApiKeyInput = chatProviders[i].showApiKeyInput;
            choosedChatProvider.value = n;
            if (n == 'Ollama') {
                if (chatProviders[i].models.find(d => d.value == settings.chatProvider.provider.model) == null) {
                    addAnotherChatOllamaModel(settings.chatProvider.provider.model);
                }
            }
            chatModelOptions.splice(0, chatModelOptions.length, ...chatProviders[i].models)
            // console.log(modelOptions.length)
            break;
        }
    }
}
const textGenerationModelOptions = reactive([])
const textGenerationDynamicReqUrlMap = new Map();
const choosedTextGenerationProvider = ref('')
const changeTextGenerationProvider = (n) => {
    if (choosedTextGenerationProvider.value)
        textGenerationDynamicReqUrlMap.set(choosedTextGenerationProvider.value, settings.textGenerationProvider.apiUrl);
    for (let i = 0; i < textGenerationProviders.length; i++) {
        if (textGenerationProviders[i].id == n) {
            if (textGenerationProviders[i].apiUrlDisabled)
                settings.textGenerationProvider.apiUrl = textGenerationProviders[i].apiUrl;
            else {
                settings.textGenerationProvider.apiUrl = sentenceEmbeddingDynamicReqUrlMap.get(settings.textGenerationProvider.provider.id);
                if (!settings.textGenerationProvider.apiUrl)
                    settings.textGenerationProvider.apiUrl = textGenerationProviders[i].apiUrl;
            }
            settings.textGenerationProvider.apiUrlDisabled = textGenerationProviders[i].apiUrlDisabled;
            settings.textGenerationProvider.showApiKeyInput = textGenerationProviders[i].showApiKeyInput;
            choosedTextGenerationProvider.value = n;
            if (n == 'Ollama') {
                if (textGenerationProviders[i].models.find(d => d.value == settings.textGenerationProvider.provider.model) == null) {
                    addAnotherTextGenerationOllamaModel(settings.textGenerationProvider.provider.model);
                }
            }
            textGenerationModelOptions.splice(0, textGenerationModelOptions.length, ...textGenerationProviders[i].models)
            // console.log(modelOptions.length)
            break;
        }
    }
}
const sentenceEmbeddingModelOptions = reactive([])
const sentenceEmbeddingDynamicReqUrlMap = new Map();
const choosedSentenceEmbeddingProvider = ref('')
const changeSentenceEmbeddingProvider = (n) => {
    if (choosedSentenceEmbeddingProvider.value)
        sentenceEmbeddingDynamicReqUrlMap.set(choosedSentenceEmbeddingProvider.value, settings.sentenceEmbeddingProvider.apiUrl);
    for (let i = 0; i < sentenceEmbeddingProviders.length; i++) {
        if (sentenceEmbeddingProviders[i].id == n) {
            if (sentenceEmbeddingProviders[i].apiUrlDisabled)
                settings.sentenceEmbeddingProvider.apiUrl = sentenceEmbeddingProviders[i].apiUrl;
            else {
                settings.sentenceEmbeddingProvider.apiUrl = sentenceEmbeddingDynamicReqUrlMap.get(settings.sentenceEmbeddingProvider.provider.id);
                if (!settings.sentenceEmbeddingProvider.apiUrl)
                    settings.sentenceEmbeddingProvider.apiUrl = sentenceEmbeddingProviders[i].apiUrl;
            }
            settings.sentenceEmbeddingProvider.apiUrlDisabled = sentenceEmbeddingProviders[i].apiUrlDisabled;
            settings.sentenceEmbeddingProvider.showApiKeyInput = sentenceEmbeddingProviders[i].showApiKeyInput;
            choosedSentenceEmbeddingProvider.value = n;
            if (n == 'Ollama') {
                if (sentenceEmbeddingProviders[i].models.find(d => d.value == settings.sentenceEmbeddingProvider.provider.model) == null) {
                    addAnotherSentenceEmbeddingOllamaModel(settings.sentenceEmbeddingProvider.provider.model);
                }
            }
            sentenceEmbeddingModelOptions.splice(0, sentenceEmbeddingModelOptions.length, ...sentenceEmbeddingProviders[i].models)
            // console.log(modelOptions.length)
            break;
        }
    }
}

const sentenceEmbeddingModelSelector = ref()
const isAddingAnotherSentenceEmbeddingOllamaModel = ref(false)
const anotherSentenceEmbeddingOllamaModel = ref('')
const addAnotherSentenceEmbeddingOllamaModel = (m) => {
    const obj = { label: m, value: m };
    sentenceEmbeddingModelOptions.unshift(obj);
    sentenceEmbeddingProviders[2].models.unshift(obj);
    sentenceEmbeddingModelSelector.value.blur();
    settings.sentenceEmbeddingProvider.provider.id = 'Ollama';
    settings.sentenceEmbeddingProvider.provider.model = obj.value;
    anotherSentenceEmbeddingOllamaModel.value = '';
}

// TTS

// https://docs.spring.io/spring-ai/reference/api/embeddings.html
const ttsProviders = [
    {
        id: 'HuggingFace',
        name: 'HuggingFace',
        apiUrl: 'Model will be downloaded locally at ./data/models',
        apiUrlDisabled: true,
        showApiKeyInput: false,
        models: [
            { label: 'parler-tts/parler-tts-mini-v1 (English 3.51GB)', value: 'ParlerTtsMiniV1' },
            { label: 'parler-tts/parler-tts-large-v1 (English 9.35GB)', value: 'ParlerTtsLargeV1' },
        ]
    },
    // {
    //     id: 'ChatTTS',
    //     name: 'ChatTTS',
    //     apiUrl: 'http://localhost:11434/api/embeddings',
    //     apiUrlDisabled: false,
    //     showApiKeyInput: false,
    //     models: [
    //         { label: 'nomic-embed-text:v1.5', value: 'nomic-embed-text:v1.5' },
    //         { label: 'mxbai-embed-large:335m', value: 'mxbai-embed-large:335m' },
    //         { label: 'snowflake-arctic-embed:335m', value: 'snowflake-arctic-embed:335m' },
    //         { label: 'jina-embeddings-v2-base-en', value: 'jina/jina-embeddings-v2-base-en:latest' },
    //     ],
    // },
    // {
    //     id: 'MicrosoftTTS',
    //     name: 'MicrosoftTTS',
    //     apiUrl: 'http://localhost:11434/api/embeddings',
    //     apiUrlDisabled: false,
    //     showApiKeyInput: false,
    //     models: [
    //         { label: 'nomic-embed-text:v1.5', value: 'nomic-embed-text:v1.5' },
    //         { label: 'mxbai-embed-large:335m', value: 'mxbai-embed-large:335m' },
    //         { label: 'snowflake-arctic-embed:335m', value: 'snowflake-arctic-embed:335m' },
    //         { label: 'jina-embeddings-v2-base-en', value: 'jina/jina-embeddings-v2-base-en:latest' },
    //     ],
    // },
]
const ttsModelOptions = reactive([])
const ttsDynamicReqUrlMap = new Map();
const choosedTtsProvider = ref('')
const changeTtsProvider = (n) => {
    if (choosedTtsProvider.value)
        ttsDynamicReqUrlMap.set(choosedTtsProvider.value, settings.ttsProvider.apiUrl);
    for (let i = 0; i < ttsProviders.length; i++) {
        if (ttsProviders[i].id == n) {
            ttsModelOptions.splice(0, ttsModelOptions.length, ...ttsProviders[i].models)
            // console.log(modelOptions.length)
            break;
        }
    }
}

const usedByLlmChatNodeBig = [chatPic]
const usedByTextGenerationBig = [textGenerationPic]
const usedBySentenceEmbeddingBig = [sentenceEmbeddingPic];
</script>
<template>
    <!-- <el-page-header :title="$t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3">{{ $t('lang.settings.title') }}</span>
        </template>
    </el-page-header> -->
    <h1>{{ $t('lang.settings.title') }}</h1>
    <h3>Common settings</h3>
    <el-row>
        <el-col :span="12" :offset="1">
            <el-form :model="settings">
                <el-form-item :label="$t('lang.settings.prompt3')" :label-width="formLabelWidth">
                    <el-input-number v-model="maxSessionIdleMin" :min="2" :max="1440" @change="handleChange" />
                    {{ $t('lang.settings.prompt4') }}
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
    <h3>
        Chat bot
        <el-tooltip effect="light" placement="right">
            <template #content>
                You don’t need to download the large model file unless you want to use the functionalities
                described below.
                <br />
                Currently, its function is merely to provide automatic response capabilities and suggested
                reply templates for dialogue nodes.
            </template>
            <el-button circle>?</el-button>
        </el-tooltip>
    </h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <el-form :model="settings.chatProvider" :label-width="formLabelWidth" style="max-width: 600px">
                <el-form-item label="Provider">
                    <el-radio-group v-model="settings.chatProvider.provider.id" size="large"
                        @change="changeChatProvider">
                        <el-radio-button v-for="item in chatProviders" :id="item.id" :key="item.id" :label="item.id"
                            :value="item.id" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Request address">
                    <el-input v-model="settings.chatProvider.apiUrl" :disabled="settings.chatProvider.apiUrlDisabled" />
                </el-form-item>
                <el-form-item label="OpenAI API key" v-show="settings.chatProvider.showApiKeyInput">
                    <el-input v-model="settings.chatProvider.apiKey" />
                </el-form-item>
                <el-form-item label="Model">
                    <el-select ref="chatModelSelector" v-model="settings.chatProvider.provider.model"
                        placeholder="Choose a model">
                        <el-option v-for="item in chatModelOptions" :id="item.value" :key="item.value"
                            :label="item.label" :value="item.value" />
                        <template #footer>
                            <el-button :disabled="settings.chatProvider.provider.id != 'Ollama'"
                                v-if="!isAddingAnotherChatOllamaModel" text bg
                                @click="isAddingAnotherChatOllamaModel = true">
                                Another ollama model
                            </el-button>
                            <template v-else>
                                <el-input v-model="anotherChatOllamaModel" placeholder="input model name"
                                    style="margin-bottom: 8px;" />
                                <el-button type="primary" @click="addAnotherChatOllamaModel(anotherChatOllamaModel)">
                                    confirm
                                </el-button>
                                <el-button @click="isAddingAnotherChatOllamaModel = false">cancel</el-button>
                            </template>
                        </template>
                    </el-select>
                </el-form-item>
                <el-form-item label="Max response token">
                    <el-input-number v-model="settings.chatProvider.maxResponseTokenLength" :min="10" :max="100000"
                        :step="5" />
                </el-form-item>
                <el-form-item label="Connect timeout" v-show="settings.chatProvider.provider.id != 'HuggingFace'">
                    <el-input-number v-model="settings.chatProvider.connectTimeoutMillis" :min="100" :max="65500"
                        :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Read timeout" v-show="settings.chatProvider.provider.id != 'HuggingFace'">
                    <el-input-number v-model="settings.chatProvider.readTimeoutMillis" :min="200" :max="65500"
                        :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Proxy" v-show="settings.chatProvider.provider.id != 'HuggingFace'">
                    <el-checkbox v-model="chatProviderProxyEnabled" label="Enable" />
                    <el-input v-model="input" placeholder="http://127.0.0.1:9270"
                        :disabled="!chatProviderProxyEnabled" />
                </el-form-item>
                <el-form-item label="" v-show="showHfIncorrectGenerationModelTip">
                    HuggingFace model files were incorrect or missing, please <el-button type="primary" text
                        @click="downloadModels(settings.chatProvider.provider.model)">
                        click here to download model files from Huggingface.co
                    </el-button>, or you can download manually and put them in ./data/model/{{
                        chatModelRepository
                    }}
                </el-form-item>
                <el-form-item label="" v-show="showHfChatModelDownloadProgress">
                    Downloading: {{ downloadingUrl }}, {{ downloadingProgress }}%
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
        <el-col :span="6" :offset="1">
            <div>This is used by LLM chat node.</div>
            <!-- <img src="../../assets/usedBySentenceEmbedding-thumbnail.png" /> -->
            <el-image :src="chatPicThumbnail" :zoom-rate="1.2" :max-scale="7" :min-scale="0.2"
                :preview-src-list="usedByLlmChatNodeBig" :initial-index="4" fit="cover" />
        </el-col>
    </el-row>
    <h3>
        Text generation
        <el-tooltip effect="light" placement="right">
            <template #content>
                You don’t need to download the large model file unless you want to use the functionalities
                described below.
                <br />
                Currently, its function is merely to provide automatic response capabilities and suggested
                reply templates for dialogue nodes.
            </template>
            <el-button circle>?</el-button>
        </el-tooltip>
    </h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <el-form :model="settings.textGenerationProvider" :label-width="formLabelWidth" style="max-width: 600px">
                <el-form-item label="Provider">
                    <el-radio-group v-model="settings.textGenerationProvider.provider.id" size="large"
                        @change="changeTextGenerationProvider">
                        <el-radio-button v-for="item in textGenerationProviders" :id="item.id" :key="item.id"
                            :label="item.id" :value="item.id" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Request address">
                    <el-input v-model="settings.textGenerationProvider.apiUrl"
                        :disabled="settings.textGenerationProvider.apiUrlDisabled" />
                </el-form-item>
                <el-form-item label="OpenAI API key" v-show="settings.textGenerationProvider.showApiKeyInput">
                    <el-input v-model="settings.textGenerationProvider.apiKey" />
                </el-form-item>
                <el-form-item label="Model">
                    <el-select ref="textGenerationModelSelector"
                        v-model="settings.textGenerationProvider.provider.model" placeholder="Choose a model">
                        <el-option v-for="item in textGenerationModelOptions" :id="item.value" :key="item.value"
                            :label="item.label" :value="item.value" />
                        <template #footer>
                            <el-button :disabled="settings.textGenerationProvider.provider.id != 'Ollama'"
                                v-if="!isAddingAnotherTextGenerationOllamaModel" text bg
                                @click="isAddingAnotherTextGenerationOllamaModel = true">
                                Another ollama model
                            </el-button>
                            <template v-else>
                                <el-input v-model="anotherTextGenerationOllamaModel" placeholder="input model name"
                                    style="margin-bottom: 8px;" />
                                <el-button type="primary"
                                    @click="addAnotherTextGenerationOllamaModel(anotherTextGenerationOllamaModel)">
                                    confirm
                                </el-button>
                                <el-button @click="isAddingAnotherTextGenerationOllamaModel = false">cancel</el-button>
                            </template>
                        </template>
                    </el-select>
                </el-form-item>
                <el-form-item label="Max response token">
                    <el-input-number v-model="settings.textGenerationProvider.maxResponseTokenLength" :min="10"
                        :max="100000" :step="5" />
                </el-form-item>
                <el-form-item label="Connect timeout"
                    v-show="settings.textGenerationProvider.provider.id != 'HuggingFace'">
                    <el-input-number v-model="settings.textGenerationProvider.connectTimeoutMillis" :min="100"
                        :max="65500" :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Read timeout"
                    v-show="settings.textGenerationProvider.provider.id != 'HuggingFace'">
                    <el-input-number v-model="settings.textGenerationProvider.readTimeoutMillis" :min="1000"
                        :max="65500" :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Proxy" v-show="settings.textGenerationProvider.provider.id != 'HuggingFace'">
                    <el-checkbox v-model="textGenerationProviderProxyEnabled" label="Enable" />
                    <el-input v-model="input" placeholder="http://127.0.0.1:9270"
                        :disabled="!textGenerationProviderProxyEnabled" />
                </el-form-item>
                <el-form-item label="" v-show="showHfIncorrectGenerationModelTip">
                    HuggingFace model files were incorrect or missing, please <el-button type="primary" text
                        @click="downloadModels(settings.textGenerationProvider.provider.model)">
                        click here to download model files from Huggingface.co
                    </el-button>, or you can download manually and put them in ./data/model/{{
                        textGenerationModelRepository
                    }}
                </el-form-item>
                <el-form-item label="" v-show="showHfGenerationModelDownloadProgress">
                    Downloading: {{ downloadingUrl }}, {{ downloadingProgress }}%
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
        <el-col :span="6" :offset="1">
            <div>This is used by dialog node.</div>
            <!-- <img src="../../assets/usedBySentenceEmbedding-thumbnail.png" /> -->
            <el-image :src="textGenerationPicThumbnail" :zoom-rate="1.2" :max-scale="7" :min-scale="0.2"
                :preview-src-list="usedByTextGenerationBig" :initial-index="4" fit="cover" />
        </el-col>
    </el-row>
    <h3>
        Sentence embedding provider
        <el-tooltip effect="light" placement="right">
            <template #content>
                Downloading model files is not necessary.<br />
                Its function is merely to enhance the accuracy of intent recognition for user inputs, and it will
                not
                affect the response functionality of the process.<br />
                User intent can also be recognized through the configuration of keywords and regular expressions
                without
                downloading the model.
            </template>
            <el-button circle>?</el-button>
        </el-tooltip>
    </h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <el-form :model="settings.sentenceEmbeddingProvider" :label-width="formLabelWidth" style="max-width: 600px">
                <el-form-item label="Provider">
                    <el-radio-group v-model="settings.sentenceEmbeddingProvider.provider.id" size="large"
                        @change="changeSentenceEmbeddingProvider">
                        <el-radio-button v-for="item in sentenceEmbeddingProviders" :id="item.id" :key="item.id"
                            :label="item.id" :value="item.id" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Request address">
                    <el-input v-model="settings.sentenceEmbeddingProvider.apiUrl"
                        :disabled="settings.sentenceEmbeddingProvider.apiUrlDisabled" />
                </el-form-item>
                <el-form-item label="OpenAI API key" v-show="settings.sentenceEmbeddingProvider.showApiKeyInput">
                    <el-input v-model="settings.sentenceEmbeddingProvider.apiKey" />
                </el-form-item>
                <el-form-item label="Model">
                    <el-select ref="sentenceEmbeddingModelSelector"
                        v-model="settings.sentenceEmbeddingProvider.provider.model" placeholder="Choose a model">
                        <el-option v-for="item in sentenceEmbeddingModelOptions" :id="item.value" :key="item.value"
                            :label="item.label" :value="item.value" />
                        <template #footer>
                            <el-button :disabled="settings.sentenceEmbeddingProvider.provider.id != 'Ollama'"
                                v-if="!isAddingAnotherSentenceEmbeddingOllamaModel" text bg
                                @click="isAddingAnotherSentenceEmbeddingOllamaModel = true">
                                Another ollama model
                            </el-button>
                            <template v-else>
                                <el-input v-model="anotherSentenceEmbeddingOllamaModel" placeholder="input model name"
                                    style="margin-bottom: 8px;" />
                                <el-button type="primary"
                                    @click="addAnotherSentenceEmbeddingOllamaModel(anotherSentenceEmbeddingOllamaModel)">
                                    confirm
                                </el-button>
                                <el-button
                                    @click="isAddingAnotherSentenceEmbeddingOllamaModel = false">cancel</el-button>
                            </template>
                        </template>
                    </el-select>
                </el-form-item>
                <el-form-item label="Similarity threshold">
                    ≥<el-input-number v-model="similarityThreshold" :min="1" :max="99" :step="1" />%
                    <el-tooltip effect="light" placement="right">
                        <template #content>
                            An intent is used when the expression matching similarity exceeds the threshold.
                        </template>
                        <el-button circle>?</el-button>
                    </el-tooltip>
                </el-form-item>
                <el-form-item label="Connect timeout"
                    v-show="settings.sentenceEmbeddingProvider.provider.id != 'HuggingFace'">
                    <el-input-number v-model="settings.sentenceEmbeddingProvider.connectTimeoutMillis" :min="100"
                        :max="65500" :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Read timeout"
                    v-show="settings.sentenceEmbeddingProvider.provider.id != 'HuggingFace'">
                    <el-input-number v-model="settings.sentenceEmbeddingProvider.readTimeoutMillis" :min="500"
                        :max="65500" :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Proxy" v-show="settings.sentenceEmbeddingProvider.provider.id != 'HuggingFace'">
                    <el-checkbox v-model="sentenceEmbeddingProviderProxyEnabled" label="Enable" />
                    <el-input v-model="input" placeholder="http://127.0.0.1:9270"
                        :disabled="!sentenceEmbeddingProviderProxyEnabled" />
                </el-form-item>
                <el-form-item label="" v-show="showHfIncorrectEmbeddingModelTip">
                    HuggingFace model files were incorrect or missing, please <el-button type="primary" text
                        @click="downloadModels(settings.sentenceEmbeddingProvider.provider.model)">
                        click here to download model files from Huggingface.co
                    </el-button>, or you can download manually and put them in ./data/model/{{
                        sentenceEmbeddingModelRepository }}
                </el-form-item>
                <el-form-item label="" v-show="showHfEmbeddingModelDownloadProgress">
                    Downloading: {{ downloadingUrl }}, {{ downloadingProgress }}%
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
        <el-col :span="6" :offset="1">
            <div>This is used by intention similar sentences.</div>
            <!-- <img src="../../assets/usedBySentenceEmbedding-thumbnail.png" /> -->
            <el-image :src="sentenceEmbeddingPicThumbnail" :zoom-rate="1.2" :max-scale="7" :min-scale="0.2"
                :preview-src-list="usedBySentenceEmbeddingBig" :initial-index="4" fit="cover" />
        </el-col>
    </el-row>
    <!-- <h3>
        Document QA
        <el-tooltip effect="light" placement="right">
            <template #content>
                Support file type: Doc, Docx, PPT, PDF, JPG, PNG, Markdown.
            </template>
            <el-button circle>?</el-button>
        </el-tooltip>
    </h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <h3>Current documents</h3>
            <el-form :model="settings.ttsProvider" :label-width="formLabelWidth" style="max-width: 600px">
                <el-form-item label="Upload">
                    <el-select v-model="value">
                        <el-option label="Dialog flow API" value="Normal" />
                        <el-option label="Slack robot" value="Normal" />
                        <el-option label="TTS Stream" value="Normal" :disabled="robotType == 'TextBot'" />
                    </el-select>
                </el-form-item>
                <el-form-item label="Enable">
                    <el-switch v-model="settings.ttsProvider.enabled" active-text="Response TTS stream"
                        inactive-text="Response text" />
                </el-form-item>
                <el-form-item label="Provider" v-show="settings.ttsProvider.enabled" @change="changeTtsProvider">
                    <el-radio-group v-model="settings.ttsProvider.provider.id" size="large">
                        <el-radio-button v-for="item in ttsProviders" :id="item.id" :key="item.id" :label="item.id"
                            :value="item.id" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Model" v-show="settings.ttsProvider.enabled">
                    <el-select ref="ttsModelSelector" v-model="settings.ttsProvider.provider.model"
                        placeholder="Choose a model">
                        <el-option v-for="item in ttsModelOptions" :id="item.value" :key="item.value"
                            :label="item.label" :value="item.value" />
                    </el-select>
                </el-form-item>
                <el-form-item label="" v-show="showHfIncorrectEmbeddingModelTip">
                    HuggingFace model files were incorrect or missing, please <el-button type="primary" text
                        @click="downloadModels(settings.sentenceEmbeddingProvider.provider.model)">
                        click here to download model files from Huggingface.co
                    </el-button>, or you can download manually and put them in ./data/model/{{
                        sentenceEmbeddingModelRepository }}
                </el-form-item>
                <el-form-item label="" v-show="showHfEmbeddingModelDownloadProgress">
                    Downloading: {{ downloadingUrl }}, {{ downloadingProgress }}%
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
    <h3>
        Response adapter
        <el-tooltip effect="light" placement="right">
            <template #content>
                Downloading model files is not necessary.<br />
                Its function is merely to enhance the accuracy of intent recognition for user inputs, and it will
                not
                affect the response functionality of the process.<br />
                User intent can also be recognized through the configuration of keywords and regular expressions
                without
                downloading the model.
            </template>
            <el-button circle>?</el-button>
        </el-tooltip>
    </h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <el-form :model="settings.ttsProvider" :label-width="formLabelWidth" style="max-width: 600px">
                <el-form-item label="Adapter">
                    <el-select v-model="value">
                        <el-option label="Dialog flow API" value="Normal" />
                        <el-option label="Slack robot" value="Normal" />
                        <el-option label="TTS Stream" value="Normal" :disabled="robotType == 'TextBot'" />
                    </el-select>
                </el-form-item>
                <el-form-item label="Enable">
                    <el-switch v-model="settings.ttsProvider.enabled" active-text="Response TTS stream"
                        inactive-text="Response text" />
                </el-form-item>
                <el-form-item label="Provider" v-show="settings.ttsProvider.enabled" @change="changeTtsProvider">
                    <el-radio-group v-model="settings.ttsProvider.provider.id" size="large">
                        <el-radio-button v-for="item in ttsProviders" :id="item.id" :key="item.id" :label="item.id"
                            :value="item.id" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="Model" v-show="settings.ttsProvider.enabled">
                    <el-select ref="ttsModelSelector" v-model="settings.ttsProvider.provider.model"
                        placeholder="Choose a model">
                        <el-option v-for="item in ttsModelOptions" :id="item.value" :key="item.value"
                            :label="item.label" :value="item.value" />
                    </el-select>
                </el-form-item>
                <el-form-item label="" v-show="showHfIncorrectEmbeddingModelTip">
                    HuggingFace model files were incorrect or missing, please <el-button type="primary" text
                        @click="downloadModels(settings.sentenceEmbeddingProvider.provider.model)">
                        click here to download model files from Huggingface.co
                    </el-button>, or you can download manually and put them in ./data/model/{{
                        sentenceEmbeddingModelRepository }}
                </el-form-item>
                <el-form-item label="" v-show="showHfEmbeddingModelDownloadProgress">
                    Downloading: {{ downloadingUrl }}, {{ downloadingProgress }}%
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row> -->
    <h3>Email settings</h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <el-form :model="settings">
                <el-form-item label="Email SMTP" :label-width="formLabelWidth">
                </el-form-item>
                <el-form-item label="Host" :label-width="formLabelWidth">
                    <el-input v-model="settings.smtpHost" placeholder="" />
                </el-form-item>
                <el-form-item label="Username" :label-width="formLabelWidth">
                    <el-input v-model="settings.smtpUsername" placeholder="" />
                </el-form-item>
                <el-form-item label="Password" :label-width="formLabelWidth">
                    <el-input v-model="settings.smtpPassword" placeholder="" type="password" />
                </el-form-item>
                <el-form-item label="Timeout" :label-width="formLabelWidth">
                    <el-input-number v-model="settings.smtpTimeoutSec" :min="1" :max="600" @change="handleChange" />
                    Seconds
                </el-form-item>
                <el-form-item label="Email verification regex" label-width="200px">
                    <el-input v-model="settings.emailVerificationRegex" :placeholder="defaultEmailVerificationRegex" />
                </el-form-item>
                <el-form-item label="" label-width="200px">
                    You can customize the email verification regular expression, or leave it blank and the system will
                    automatically use the general verification rules.
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button :loading="loading" type="info" @click="smtpTest">
                        Test SMTP settings
                    </el-button>
                    <el-alert v-if="smtpPassed" title="SMTP test passed" type="success" />
                    <el-alert v-if="smtpFailed" :title="smtpFailedDetail" type="error" />
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.back') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
</template>