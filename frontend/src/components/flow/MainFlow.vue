<script setup>
import { ref, reactive, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { btoa, httpReq } from '../../assets/tools.js'
import { useI18n } from 'vue-i18n'
// import { ElMessage, ElMessageBox } from 'element-plus';
const { t, tm, rt } = useI18n();
const route = useRoute();
const router = useRouter();
const robotId = route.params.robotId;
const mainFlowData = reactive({
    _idx: 0,
    id: '',
    name: '',
    enabled: true,
});

const setFormVisible = ref(false);
const formLabelWidth = '130px';
const tableData = ref([])

onMounted(async () => {
    const t = await httpReq('GET', 'mainflow', { robotId: robotId }, null, null);
    // console.log(t);
    showMainFlows(t);
});

const showMainFlows = (t) => {
    if (t && t.status == 200) {
        tableData.value = t.data == null ? [] : t.data;
    }
}

const goBack = () => {
    router.push({ name: 'robotDetail', params: { robotId: robotId } });
}

const toSubflow = (idx, d) => {
    // console.log(d.name);
    router.push({ name: 'subflow', params: { robotId: robotId, id: d.id, name: btoa(d.name) } })
}

const newMainFlow = () => {
    mainFlowData.id = ''
    mainFlowData.name = ''
    mainFlowData.enabled = true
    showForm()
}

const editMainFlow = (idx, d) => {
    // console.log(idx);
    mainFlowData._idx = idx;
    mainFlowData.id = d.id
    mainFlowData.name = d.name
    mainFlowData.enabled = d.enabled
    showForm()
}

const deleteMainFlow = async (idx, d) => {
    ElMessageBox.confirm(
        t('lang.mainflow.delConfirm'),
        'Warning',
        {
            confirmButtonText: t('lang.common.del'),
            cancelButtonText: t('lang.common.cancel'),
            type: 'warning',
        }
    ).then(async () => {
        mainFlowData.id = d.id
        const t = await httpReq('DELETE', 'mainflow', { robotId: robotId }, null, mainFlowData);
        // console.log(t);
        tableData.value.splice(idx, 1);
        hideForm();
        ElMessage({
            type: 'success',
            message: t('lang.common.deleted'),
        })
    }).catch(() => {
        // ElMessage({
        //     type: 'info',
        //     message: 'Delete canceled',
        // })
    })
}

function showForm() {
    setFormVisible.value = true;
}

function hideForm() {
    setFormVisible.value = false;
}

const saveForm = async () => {
    const editRecord = mainFlowData.id;
    const r = await httpReq(editRecord ? 'PUT' : 'POST', 'mainflow', { robotId: robotId }, null, mainFlowData);
    // console.log(r);
    if (editRecord) {
        console.log(mainFlowData._idx, mainFlowData, mainFlowData.value);
        tableData.value[mainFlowData._idx] = {
            _idx: mainFlowData._idx,
            id: mainFlowData.id,
            name: mainFlowData.name,
            enabled: mainFlowData.enabled,
        };
    } else {
        if (r.status == 200)
            tableData.value.push(r.data);
    }
    hideForm();
};
</script>
<style scoped></style>
<template>
    <!-- <el-page-header :title="t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3"> {{ $t('lang.mainflow.title') }} </span>
        </template>
        <template #extra>
            <div class="flex items-center">
                <el-button type="primary" class="ml-2" @click="newMainFlow()">{{ $t('lang.mainflow.add') }}</el-button>
            </div>
        </template>
    </el-page-header> -->
    <h1>{{ $t('lang.mainflow.title') }}</h1>
    <el-button type="primary" class="ml-2" @click="newMainFlow()">{{ $t('lang.mainflow.add') }}</el-button>
    <el-table :data="tableData" stripe style="width: 100%">
        <el-table-column prop="id" label="Id" width="240" />
        <el-table-column prop="name" :label="tm('lang.mainflow.table')[0]" width="500" />
        <!-- <el-table-column prop="enabled" :label="tm('lang.mainflow.table')[1]" width="80" /> -->
        <el-table-column fixed="right" :label="tm('lang.mainflow.table')[2]" min-width="40">
            <template #default="scope">
                <el-button link type="primary" @click="toSubflow(scope.$index, scope.row)">
                    {{ $t('lang.common.edit') }}
                </el-button> |
                <el-button link type="primary" @click="editMainFlow(scope.$index, scope.row)">
                    {{ $t('lang.common.edit') }} name
                </el-button> |
                <el-button link type="danger" @click="deleteMainFlow(scope.$index, scope.row)">
                    {{ $t('lang.common.del') }}
                </el-button>
            </template>
        </el-table-column>
    </el-table>
    <el-dialog v-model="setFormVisible" :title="$t('lang.mainflow.form.title')" width="60%">
        <el-form :model="nodeData">
            <el-form-item :label="$t('lang.mainflow.form.name')" :label-width="formLabelWidth">
                <el-input v-model="mainFlowData.name" autocomplete="off" />
            </el-form-item>
        </el-form>
        <template #footer>
            <el-button type="primary" :loading="loading" @click="saveForm()">{{ $t('lang.common.save') }}</el-button>
            <el-button @click="hideForm()">{{ $t('lang.common.cancel') }}</el-button>
        </template>
    </el-dialog>
</template>