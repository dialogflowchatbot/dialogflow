<script setup>
import { ref, reactive, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
// import { ElMessage, ElMessageBox } from 'element-plus'
import { copyProperties, httpReq } from '../../assets/tools.js'
import { useI18n } from 'vue-i18n'
const { t, tm, rt } = useI18n();
const route = useRoute();
const router = useRouter();
const robotId = route.params.robotId;
const varData = reactive({
    varName: '',
    varType: '',
    varValueSource: '',
    varConstantValue: '',
    varAssociateData: '',
    obtainValueExpressionType: 'None',
    obtainValueExpression: '',
    cacheEnabled: true,
});
const varTypes = [
    { label: tm('lang.var.types')[0], value: 'Str' },
    { label: tm('lang.var.types')[1], value: 'Num' },
];
const varTypesMap = new Map()
varTypes.forEach(function (item, index, arr) {
    this.set(item.value, item.label);
}, varTypesMap);

const varValueSources = [
    { label: tm('lang.var.sources')[0], value: 'Import', disabled: false },
    { label: tm('lang.var.sources')[1], value: 'Collect', disabled: false },
    { label: 'User input', value: 'UserInput', disabled: false },
    { label: 'Constant value', value: 'Constant', disabled: false },
    { label: tm('lang.var.sources')[2], value: 'ExternalHttp', disabled: false },
];
const varValueSourcesMap = new Map()
varValueSources.forEach(function (item, index, arr) {
    this.set(item.value, item.label);
}, varValueSourcesMap);

const obtainValueExpressionTypes = [
    { label: 'JSON Pointer', value: 'JsonPointer', disabled: false },
    { label: 'Html Scrape', value: 'HtmlScrape', disabled: false },
]

const varSetFormVisible = ref(false);
const formLabelWidth = '160px';
const tableData = ref([])
const httpApiList = ref([])

async function list() {
    const t = await httpReq('GET', 'variable', { robotId: robotId }, null, null);
    console.log(t);
    showVars(t);
}

onMounted(async () => {
    const t = await httpReq('GET', 'external/http', { robotId: robotId }, null, null);
    // console.log(t);
    if (t && t.status == 200) {
        httpApiList.value = t.data == null ? [] : t.data;
    }
    await list();
});

const showVars = (t) => {
    if (t && t.status == 200) {
        tableData.value = t.data == null ? [] : t.data;
        tableData.value.forEach(function (item, index, arr) {
            item.varTypeT = varTypesMap.get(item.varType);
            item.varValueSourceT = varValueSourcesMap.get(item.varValueSource);
        });
    }
}

const goBack = () => {
    router.push({ name: 'robotDetail', params: { robotId: robotId } });
}

const newVar = () => {
    varData.varName = ''
    varData.varType = ''
    varData.varValueSource = ''
    varData.constantValue = ''
    varData.externalAssociateId = ''
    varData.obtainValueExpressionType = 'None'
    varData.obtainValueExpression = ''
    varData.cacheEnabled = false
    showForm()
}

const editVar = (idx, d) => {
    copyProperties(d, varData);
    // varData.varName = d.varName
    // varData.varType = d.varType
    // varData.varValueSource = d.varValueSource
    // varData.externalAssociateId = d.externalAssociateId
    // varData.obtainValueExpressionType = d.obtainValueExpressionType
    // varData.obtainValueExpression = d.obtainValueExpression
    // varData.cacheEnabled = d.cacheEnabled
    showForm()
}

const deleteVar = async (idx, d) => {
    ElMessageBox.confirm(
        d.varName + ' will be deleted permanently. Continue?',
        'Warning',
        {
            confirmButtonText: 'OK',
            cancelButtonText: 'Cancel',
            type: 'warning',
        }
    )
        .then(async () => {
            copyProperties(d, varData);
            // varData.varName = idx.toString();
            // varData.varType = d.varType
            // varData.varValueSource = d.varValueSource
            const t = await httpReq('DELETE', 'variable', { robotId: robotId }, null, varData);
            console.log(t);
            if (t.status == 200) {
                await list();
                // hideForm();
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

function showForm() {
    varSetFormVisible.value = true;
}

function hideForm() {
    varSetFormVisible.value = false;
}

async function saveForm() {
    const t = await httpReq('POST', 'variable', { robotId: robotId }, null, varData);
    console.log(t);
    await list();
    hideForm();
}
</script>
<style scoped></style>
<template>
    <!-- <el-page-header :title="t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3"> {{ $t('lang.var.title') }} </span>
        </template>
        <template #extra>
            <div class="flex items-center">
                <el-button type="primary" class="ml-2" @click="newVar()">{{ $t('lang.var.add') }}</el-button>
            </div>
        </template>
    </el-page-header> -->
    <h1>{{ $t('lang.var.title') }}</h1>
    <el-button type="primary" class="ml-2" @click="newVar()">{{ $t('lang.var.add') }}</el-button>
    <el-table :data="tableData" stripe style="width: 100%">
        <el-table-column prop="varName" :label="tm('lang.var.table')[0]" width="300" />
        <el-table-column prop="varTypeT" :label="tm('lang.var.table')[1]" width="180" />
        <el-table-column prop="varValueSourceT" :label="tm('lang.var.table')[2]" width="200" />
        <el-table-column fixed="right" :label="tm('lang.var.table')[3]" min-width="40">
            <template #default="scope">
                <el-button link type="primary" @click="editVar(scope.$index, scope.row)">
                    {{ $t('lang.common.edit') }}
                </el-button>
                <el-button link type="danger" @click="deleteVar(scope.$index, scope.row)">
                    {{ $t('lang.common.del') }}
                </el-button>
            </template>
        </el-table-column>
    </el-table>
    <el-drawer v-model="varSetFormVisible" :title="$t('lang.var.form.title')" direction="rtl" size="50%">
        <el-form :model="nodeData">
            <el-form-item :label="$t('lang.var.form.name')" :label-width="formLabelWidth">
                <el-input v-model="varData.varName" autocomplete="off" />
            </el-form-item>
            <el-form-item :label="$t('lang.var.form.type')" :label-width="formLabelWidth">
                <el-select v-model="varData.varType" :placeholder="$t('lang.var.form.choose1')">
                    <el-option v-for="item in varTypes" :key="item.label" :label="item.label" :value="item.value"
                        :disabled="item.disabled" />
                </el-select>
            </el-form-item>
            <el-form-item :label="$t('lang.var.form.source')" :label-width="formLabelWidth">
                <el-select v-model="varData.varValueSource" :placeholder="$t('lang.var.form.choose2')">
                    <el-option v-for="item in varValueSources" :key="item.label" :label="item.label"
                        :value="item.value" />
                </el-select>
            </el-form-item>
            <el-form-item v-if="varData.varValueSource == 'Constant'" label="Constant value"
                :label-width="formLabelWidth">
                <el-input v-model="varData.varConstantValue" autocomplete="on" />
            </el-form-item>
            <el-form-item v-if="varData.varValueSource == 'ExternalHttp'" label="HTTP API"
                :label-width="formLabelWidth">
                <el-select v-model="varData.varAssociateData" placeholder="Choose a HTTP API">
                    <el-option v-for="item in httpApiList" :key="item.id" :label="item.name" :value="item.id" />
                </el-select>
                <br />
                <router-link :to="{ name: 'externalHttpApiDetail', params: { robotId: robotId, id: 'new' } }">Add new
                    HTTP
                    API</router-link>
            </el-form-item>
            <el-form-item v-if="varData.varValueSource == 'ExternalHttp'" label="Value expression type"
                :label-width="formLabelWidth">
                <el-select v-model="varData.obtainValueExpressionType" placeholder="Value expression type">
                    <el-option v-for="item in obtainValueExpressionTypes" :key="item.label" :label="item.label"
                        :value="item.value" />
                </el-select>
            </el-form-item>
            <el-form-item v-if="varData.varValueSource == 'ExternalHttp'" label="Obtain value expression"
                :label-width="formLabelWidth">
                <el-input v-model="varData.obtainValueExpression" autocomplete="on"
                    :placeholder="varData.obtainValueExpressionType == 'JsonPointer' ? '/data/book/name' : 'CSS selector syntax like: h1.foo div#bar'" />
            </el-form-item>
            <el-form-item v-if="varData.varValueSource == 'ExternalHttp'" label="Cache value"
                :label-width="formLabelWidth">
                <input type="checkbox" id="_cacheEnabled_" v-model="varData.cacheEnabled"
                    :checked="varData.cacheEnabled" /><label for="_cacheEnabled_">Enable</label>
            </el-form-item>
            <el-form-item v-if="varData.varValueSource == 'ExternalHttp'" label="" :label-width="formLabelWidth">
                <span v-if="varData.cacheEnabled">After requesting once, the variable value will be stored in the cache
                    and
                    subsequently read from the cache.</span>
                <span v-if="!varData.cacheEnabled">HTTP API will be requested every time</span>
            </el-form-item>
        </el-form>
        <div class="demo-drawer__footer">
            <el-button type="primary" :loading="loading" @click="saveForm()">{{ $t('lang.common.save') }}</el-button>
            <el-button @click="hideForm()">{{ $t('lang.common.cancel') }}</el-button>
        </div>
    </el-drawer>
</template>