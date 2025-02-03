<script setup>
import { reactive, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { httpReq } from '../../assets/tools.js'
import { UploadFilled } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'
const route = useRoute()
const router = useRouter();
const { t, tm, rt } = useI18n();
const robotId = route.params.robotId
const uploadUrlHost = window.location.href.indexOf('localhost') > -1 ? 'http://localhost:12715' : '';
const uploadUrl = uploadUrlHost + "/kb/doc/upload?robotId=" + robotId
const formLabelWidth = '80px'
const docDetailVisible = ref(false)
const editFormVisible = ref(false)
const loading = ref(false)
const docFile = reactive({
    docId: '',
    fileName: '',
    docContent: '',
    fileSizeInBytes: 0,
})
const tableData = reactive([])
/*
const updateUploadingProgress = (evt, uploadFile, uploadFiles) => {
    // const file = document.getElementById('f').files[0];
    // fetch('', {
    //     method: 'POST',
    //     body: file,
    //     onprogress: (event) => {
    //         if (event.lengthComputable) {
    //             const p = Math.round(event.loaded / event.total * 100);
    //         }
    //     },
    // });
    if (evt.lengthComputable) {
        const p = `Uploading (${((evt.loaded / evt.total) * 100).toFixed(2,)}%)…`;
        const m = 'Uploading ' + JSON.stringify(uploadFile) + ' ' + p + '%'
        console.log(m)
    }
}
*/
const listDocs = async () => {
    const t = await httpReq('GET', 'kb/doc', { robotId: robotId }, null, null);
    console.log(t);
    if (t.status == 200)
        tableData.splice(0, tableData.length, ...t.data);
}
onMounted(() => {
    listDocs();
})
const uploadSuccessful = (res, uploadFile, uploadFiles) => {
    console.log(uploadFile)
    listDocs()
}
const uploadFailed = (err, uploadFile, uploadFiles) => {
    console.log(err)
}
const showDocDetail = (idx) => {
    docFile.fileName = tableData[idx].fileName
    docFile.docContent = tableData[idx].docContent
    docDetailVisible.value = true
}
const editDoc = (idx) => {
    docFile.fileName = tableData[idx].fileName
    docFile.docContent = tableData[idx].docContent
    editFormVisible.value = true
}
const deleteDoc = (idx) => {
    ElMessageBox.confirm(
        'Confirm to delete this document?',
        'Warning',
        {
            confirmButtonText: t('lang.common.del'),
            cancelButtonText: t('lang.common.cancel'),
            type: 'warning',
        }
    ).then(async () => {
        const d = tableData[idx];
        if (d) {
            qaData.id = d.id;
            const t = await httpReq('DELETE', 'kb/doc', { robotId: robotId }, null, qaData);
            console.log(t);
            nextTick(() => {
                listDocs()
            })
        }
    }).catch(() => {
        // ElMessage({
        //     type: 'info',
        //     message: 'Delete canceled',
        // })
    })
}
const goBack = () => {
    router.push({ name: 'robotDetail', params: { robotId: robotId } });
}

</script>
<template>
    <!-- <el-page-header :title="$t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3">Documents management</span>
        </template>
</el-page-header> -->
    <h1>Documents</h1>
    <el-upload drag :action="uploadUrl" :on-success="uploadSuccessful" :on-error="uploadFailed">
        <el-icon class="el-icon--upload"><upload-filled /></el-icon>
        <div>
            Drop file here or <em>click to upload</em>
        </div>
        <template #tip>
            <div class="el-upload__tip">
                docx files with a size less than 20MB
            </div>
        </template>
    </el-upload>
    <el-button type="primary" @click="dryRunFormVisible = true">Test docs</el-button>
    <el-table :data="tableData" style="width: 100%">
        <el-table-column prop="fileName" label="File name" width="270" />
        <el-table-column prop="fileSize" label="File size" width="150" />
        <el-table-column prop="docContent" label="File content" />
        <el-table-column fixed="right" label="Operations" width="200">
            <template #default="scope">
                <el-button link type="primary" @click="showDocDetail(scope.$index)">Detail</el-button>
                <el-button link type="primary" @click="editDoc(scope.$index)">Edit</el-button>
                <el-button link type="danger" @click="deleteDoc(scope.$index)">Delete</el-button>
            </template>
        </el-table-column>
    </el-table>
    <el-dialog v-model="docDetailVisible" :title="docFile.fileName" width="800">
        <div>{{ docFile.docContent }}</div>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="docDetailVisible = false">Close</el-button>
            </div>
        </template>
    </el-dialog>
    <el-drawer v-model="editFormVisible" :title="docFile.fileName" direction="rtl" size="70%" :append-to-body="true"
        :destroy-on-close="true">
        <el-form :model="docFile">
            <el-form-item label="Content" :label-width="formLabelWidth">
                <el-input v-model="docFile.docContent" placeholder="" type="textarea" :rows="30" />
            </el-form-item>
        </el-form>
        <div class="demo-drawer__footer">
            <el-button type="primary" :loading="loading" @click="saveDoc()">{{ t('lang.common.save') }}</el-button>
            <el-button @click="editFormVisible = false">{{ t('lang.common.cancel') }}</el-button>
        </div>
    </el-drawer>
</template>