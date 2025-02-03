<script setup>
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { copyProperties, httpReq } from '../../assets/tools.js'
// import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n'
const { t, tm } = useI18n();
const router = useRouter();

const goBack = () => {
    router.push('/')
}

const settings = reactive({
    ip: '127.0.0.1',
    port: '12715',
    selectRandomPortWhenConflict: false,
    hfModelDownload: {
        connectTimeoutMillis: 1000,
        readTimeoutMillis: 10000,
        accessToken: '',
    }
});
const formLabelWidth = '130px'

onMounted(async () => {
    const t = await httpReq("GET", 'management/global-settings', null, null, null)
    console.log(t);
    if (t.status == 200) {
        copyProperties(t.data, settings);
        // const d = t.data;
        // settings.port = d.port;
        // settings.maxSessionDurationMin = d.maxSessionDurationMin;
    }
})

async function save() {
    let r = await httpReq("POST", 'management/global-settings', null, null, settings)
    console.log(r);
    if (r.status == 200) {
        ElMessage({ type: 'success', message: t('lang.common.saved'), });
        await checkHfModelFiles();
    } else {
        const m = t(r.err.message);
        ElMessage.error(m ? m : r.err.message);
    }
}
</script>
<template>
    <el-page-header :title="$t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3">{{ $t('lang.settings.title') }}</span>
        </template>
    </el-page-header>
    <h3>Common settings</h3>
    <el-row>
        <el-col :span="12" :offset="1">
            <el-form :model="settings">
                <el-form-item label="Listening IP addr (v4 or v6)" :label-width="formLabelWidth">
                    <el-input v-model="settings.ip" placeholder="" />
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    {{ $t('lang.settings.ipNote') }}
                </el-form-item>
                <el-form-item :label="$t('lang.settings.prompt2')" :label-width="formLabelWidth">
                    <el-input-number v-model="settings.port" :min="1024" :max="65530" @change="handleChange" />
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <input type="checkbox" id="_randomPortWhenConflict_" v-model="settings.selectRandomPortWhenConflict"
                        :checked="settings.selectRandomPortWhenConflict" />
                    <label for="_randomPortWhenConflict_">{{ $t('lang.settings.prompt2_2') }}</label>
                </el-form-item>
                <el-form-item :label-width="formLabelWidth">
                    {{ $t('lang.settings.note') }}
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.cancel') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
    <h3>HuggingFace model downloading settings</h3>
    <el-row>
        <el-col :span="11" :offset="1">
            <el-form :model="settings.modelDownload" :label-width="formLabelWidth" style="max-width: 600px">
                <el-form-item label="Connect timeout">
                    <el-input-number v-model="settings.hfModelDownload.connectTimeoutMillis" :min="100" :max="50000"
                        :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Read timeout">
                    <el-input-number v-model="settings.hfModelDownload.readTimeoutMillis" :min="1000" :max="65530"
                        :step="100" />
                    millis
                </el-form-item>
                <el-form-item label="Access token">
                    <el-input v-model="settings.hfModelDownload.accessToken" placeholder="" />
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="save">
                        {{ $t('lang.common.save') }}
                    </el-button>
                    <el-button @click="goBack()">{{ $t('lang.common.cancel') }}</el-button>
                </el-form-item>
            </el-form>
        </el-col>
    </el-row>
</template>