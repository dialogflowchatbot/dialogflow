<script setup>
import { nextTick, reactive, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
// import { ElMessage, ElMessageBox } from 'element-plus'
import { httpReq } from '../../assets/tools.js'
import { useI18n } from 'vue-i18n'
const { t, tm, rt } = useI18n();
const route = useRoute();
const router = useRouter();
const robotId = route.params.robotId;

const intentData = reactive({
    keywords: [],
    regexes: [],
    phrases: [],
});

const formData = {
    robotId: '',
    id: '',
    data: '',
};

onMounted(async () => {
    formData.robotId = robotId;
    formData.id = route.query.id;
    let t = await httpReq('GET', 'intent/detail', formData, null, null);
    console.log(t.data);
    if (t.status == 200 && t.data) {
        intentData.keywords = t.data.keywords;
        intentData.regexes = t.data.regexes;
        intentData.phrases = t.data.phrases.map((cur, idx, arr) => cur.phrase);
    }
    t = await httpReq("GET", 'management/settings/model/check/embedding', { robotId: robotId }, null, null);
    // console.log(t);
    phraseInputDisabled.value = t == null || t.status == null || t.status != 200;
    // console.log(phraseInputDisabled.value)
});

//keyword
const keywordValue = ref('');
const keywordInputVisible = ref(false);
const keywordInputRef = ref();
const showKeyWordInput = () => {
    keywordInputVisible.value = true
    nextTick(() => {
        keywordInputRef.value.focus()
    })
}

async function newKeyword() {
    if (keywordValue.value) {
        formData.id = route.query.id;
        formData.data = keywordValue.value;
        const t = await httpReq('POST', 'intent/keyword', { id: formData.id, data: route.query.idx }, null, formData);
        console.log(t.data);
        if (t.status == 200)
            intentData.keywords.push(keywordValue.value)
    }
    keywordInputVisible.value = false
    keywordValue.value = ''
}

async function removeKeyword(w) {
    ElMessageBox.confirm(
        w + ' will be deleted permanently. Continue?',
        'Warning',
        {
            confirmButtonText: 'OK',
            cancelButtonText: 'Cancel',
            type: 'warning',
        }
    )
        .then(async () => {
            const idx = intentData.keywords.indexOf(w);
            formData.id = route.query.id;
            formData.data = idx.toString();
            const t = await httpReq('DELETE', 'intent/keyword', null, null, formData);
            console.log(t.data);
            if (t.status == 200) {
                intentData.keywords.splice(idx, 1);
                ElMessage({
                    type: 'success',
                    message: 'Delete completed',
                })
            }
        })
        .catch(() => {
            // ElMessage({
            //     type: 'info',
            //     message: 'Delete canceled',
            // })
        })
}

//regex
const regexValue = ref('');
const regexInputVisible = ref(false);
const regexInputRef = ref();
const showRegexInput = () => {
    regexInputVisible.value = true
    nextTick(() => {
        regexInputRef.value.focus()
    })
}

async function newRegex() {
    if (regexValue.value) {
        formData.id = route.query.id;
        formData.data = regexValue.value;
        const t = await httpReq('POST', 'intent/regex', { id: formData.id, data: route.query.idx }, null, formData);
        console.log(t.data);
        if (t.status == 200)
            intentData.regexes.push(regexValue.value)
    }
    regexInputVisible.value = false
    regexValue.value = ''
}

async function removeRegex(w) {
    ElMessageBox.confirm(
        w + ' will be deleted permanently. Continue?',
        'Warning',
        {
            confirmButtonText: 'OK',
            cancelButtonText: 'Cancel',
            type: 'warning',
        }
    )
        .then(async () => {
            const idx = intentData.regexes.indexOf(w);
            formData.id = route.query.id;
            formData.data = idx.toString();
            const t = await httpReq('DELETE', 'intent/regex', null, null, formData);
            console.log(t.data);
            if (t.status == 200) {
                intentData.regexes.splice(idx, 1);
                ElMessage({
                    type: 'success',
                    message: 'Delete completed',
                })
            }
        })
        .catch(() => {
            // ElMessage({
            //     type: 'info',
            //     message: 'Delete canceled',
            // })
        })
}

//phrase
const phraseValue = ref('');
const phraseInputDisabled = ref(true);
const phraseInputVisible = ref(false);
const phraseInputRef = ref();
const addPhraseFailedAlertTitle = ref('')
const showAddedPhraseFailedTip = ref(false)
const regeneratingAllEmbeddings = ref(false)
const showPhraseInput = () => {
    phraseInputVisible.value = true
    nextTick(() => {
        phraseInputRef.value.focus()
    })
}

async function newPhrase() {
    if (phraseValue.value) {
        formData.id = route.query.id;
        formData.data = phraseValue.value;
        const t = await httpReq('POST', 'intent/phrase', { robotId: robotId, id: formData.id, data: route.query.idx }, null, formData);
        console.log(t.data);
        if (t.status == 200)
            intentData.phrases.push(phraseValue.value)
        else {
            addPhraseFailedAlertTitle.value = 'Added similar sentence failed: ' + t.err.message;
            // ElMessage.error(t.err.message);
            showAddedPhraseFailedTip.value = true
        }
    }
    phraseInputVisible.value = false
    phraseValue.value = ''
}

async function removePhrase(w) {
    ElMessageBox.confirm(
        w + ' will be deleted permanently. Continue?',
        'Warning',
        {
            confirmButtonText: 'OK',
            cancelButtonText: 'Cancel',
            type: 'warning',
        }
    )
        .then(async () => {
            const idx = intentData.phrases.indexOf(w);
            formData.id = route.query.id;
            formData.data = idx.toString();
            const t = await httpReq('DELETE', 'intent/phrase', null, null, formData);
            console.log(t.data);
            if (t.status == 200) {
                intentData.phrases.splice(idx, 1);
                ElMessage({
                    type: 'success',
                    message: 'Delete completed',
                })
            }
        })
        .catch(() => {
            // ElMessage({
            //     type: 'info',
            //     message: 'Delete canceled',
            // })
        })
}

const regenerateAll = async () => {
    const t = await httpReq("GET", 'management/settings', { robotId: robotId }, null, null)
    console.log(t);
    if (t.status == 200 && t.data) {
        if (t.data.sentenceEmbeddingProvider.provider.id == 'OpenAI') {
            ElMessageBox.confirm(
                'The sentence embedding providor is OpenAI, this will incur some fees. Continue?',
                'Warning',
                {
                    confirmButtonText: 'Regenerate all',
                    cancelButtonText: 'Cancel',
                    type: 'warning',
                }
            )
                .then(async () => {
                    doRegenerateAll()
                })
                .catch(() => {
                })
            return;
        }
    }
    doRegenerateAll()
}

const doRegenerateAll = async () => {
    regeneratingAllEmbeddings.value = true
    httpReq('GET', 'intent/phrase/regenerate-all', { robotId: robotId, id: formData.id, data: '' }, null, null).then(v => regeneratingAllEmbeddings.value = false);
}

const goBack = () => {
    router.push({ name: 'intents', params: { robotId: robotId } })
}
</script>
<style scoped></style>
<template>
    <el-page-header :title="t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3">{{ $t('lang.intent.detail.edit') }}: {{ route.query.name }} </span>
        </template>
    </el-page-header>

    <h3>{{ $t('lang.intent.detail.kw') }}</h3>
    <div style="color: gray;">Case insensitive</div>
    <el-tag v-for="tag in intentData.keywords" type="info" :key="tag" class="mx-1" closable :disable-transitions="false"
        @close="removeKeyword(tag)">
        {{ tag }}
    </el-tag>
    <el-input v-if="keywordInputVisible" ref="keywordInputRef" v-model="keywordValue" class="ml-1 w-20" size="small"
        @keyup.enter="newKeyword" @blur="newKeyword" />
    <el-button v-else class="button-new-tag ml-1" size="small" @click="showKeyWordInput">
        + {{ $t('lang.intent.detail.addKw') }}
    </el-button>

    <h3>{{ $t('lang.intent.detail.re') }}</h3>
    <el-tag v-for="tag in intentData.regexes" type="info" :key="tag" class="mx-1" closable :disable-transitions="false"
        @close="removeRegex(tag)">
        {{ tag }}
    </el-tag>
    <el-input v-if="regexInputVisible" ref="regexInputRef" v-model="regexValue" class="ml-1 w-20" size="small"
        @keyup.enter="newRegex" @blur="newRegex" />
    <el-button v-else class="button-new-tag ml-1" size="small" @click="showRegexInput">
        + {{ $t('lang.intent.detail.addRe') }}
    </el-button>

    <h3>{{ $t('lang.intent.detail.sp') }}</h3>
    <el-tag v-for="tag in intentData.phrases" type="info" :key="tag" class="mx-1" closable :disable-transitions="false"
        @close="removePhrase(tag)">
        {{ tag }}
    </el-tag>
    <el-input v-if="phraseInputVisible" ref="phraseInputRef" v-model="phraseValue" class="ml-1 w-20" size="small"
        @keyup.enter="newPhrase" @blur="newPhrase" />
    <el-button v-else class="button-new-tag ml-1" size="small" @click="showPhraseInput" :disabled="phraseInputDisabled">
        + {{ $t('lang.intent.detail.addSp') }}
    </el-button>
    <div v-show="phraseInputDisabled">
        This feature was disabled because <b>local model files were missing</b> or <b>api-key of OpenAI is empty</b>,
        please
        goto <router-link :to="{ name: 'settings', params: { robotId: robotId } }">settings</router-link> and select one
        model first.
    </div>
    <el-divider />
    <el-alert v-show="showAddedPhraseFailedTip" :title="addPhraseFailedAlertTitle" type="error"
        description="But don't worry, maybe you switched different embedding provider caused this. You can press 'Regenerate all similar sentences.' button below to fix this issue."
        show-icon />
    <div v-show="!phraseInputDisabled">
        <el-button type="warning" plain :loading="regeneratingAllEmbeddings" @click="regenerateAll">
            Regenerate all similar sentences.
        </el-button>
    </div>
</template>