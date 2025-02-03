<script setup>
import { ref, reactive, onMounted } from 'vue';
import { useRoute,useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n'
// import { ElMessage, ElMessageBox } from 'element-plus'
const { t, tm, rt } = useI18n();
import { btoa, httpReq } from '../../assets/tools.js'
const route=useRoute();
const router = useRouter();
const robotId=route.params.robotId

const tableData = ref([])
onMounted(async () => {
    const t = await httpReq('GET', 'external/http', {robotId:robotId}, null, null);
    // console.log(t);
    if (t && t.status == 200) {
        tableData.value = t.data == null ? [] : t.data;
    }
});

const goBack = () => {
    router.push({ name: 'robotDetail', params: { robotId: robotId } });
}
const newApi = () => {
    router.push({ name: 'externalHttpApiDetail', params: { id: 'new' } })
}
const editApi = (idx, row) => {
    router.push({ name: 'externalHttpApiDetail', params: { id: row.id } })
}
const delApi = (idx, row) => {
    ElMessageBox.confirm(
        'Confirm whether to permanently delete this record?',
        'Warning',
        {
            confirmButtonText: 'OK',
            cancelButtonText: 'Cancel',
            type: 'warning',
        }
    )
        .then(async () => {
            const t = await httpReq('DELETE', 'external/http/' + row.id, { robotId: robotId }, null, null);
            // console.log(t);
            if (t && t.status == 200) {
                ElMessage({
                    showClose: true,
                    message: 'Successfully deleted.',
                    type: 'success',
                });
                tableData.value.splice(idx, 1);
            } else {
                ElMessage({
                    showClose: true,
                    message: 'Delete failed.',
                    type: 'error',
                })
            }
        })
        .catch(() => {
        })
}
</script>
<template>
    <!-- <el-page-header :title="t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3"> External HTTP API list </span>
        </template>
        <template #extra>
            <div class="flex items-center">
                <el-button type="primary" class="ml-2" @click="newApi()">Add new external API</el-button>
            </div>
        </template>
    </el-page-header> -->
    <h1>External HTTP APIs list</h1>
    <el-button type="primary" class="ml-2" @click="newApi()">Add new external API</el-button>
    <div style="padding:10px;border: 1px solid #E6A23C; background-color: #fdf6ec;margin:10px">
        Now you can not only send data to the outside, but also get data from the outside and save it in variables
        by setting value source to a HTTP API.
        <router-link :to="{ name: 'variables', params: { robotId: robotId } }">Add new variable</router-link>
    </div>
    <el-table :data="tableData" stripe style="width: 100%">
        <el-table-column prop="name" label="HTTP name" width="450" />
        <el-table-column prop="description" label="Description" width="500" />
        <el-table-column fixed="right" :label="tm('lang.mainflow.table')[2]" min-width="40">
            <template #default="scope">
                <el-button link type="primary" @click="editApi(scope.$index, scope.row)">
                    Edit
                </el-button> |
                <el-button link type="danger" @click="delApi(scope.$index, scope.row)">
                    Delete
                </el-button>
            </template>
        </el-table-column>
    </el-table>
</template>