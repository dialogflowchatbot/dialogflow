<script setup>
import { reactive, ref, onMounted, nextTick } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { httpReq } from '../../assets/tools.js'
import { useI18n } from 'vue-i18n'
const { t, tm, rt } = useI18n();
const route = useRoute()
const router = useRouter();
const robotId = route.params.robotId
const qaData = reactive({
    id: null,
    question: {
        question: ''
    },
    similarQuestions: [],
    answer: '',
})
const tableData = reactive([])
const objectSpanMethod = ({ row, column, rowIndex, columnIndex }) => {
    console.log(column);
    return { rowspan: column.length, colspan: 1 }
}
const listQa = async () => {
    const t = await httpReq('GET', 'kb/qa', { robotId: robotId }, null, null);
    console.log(t);
    if (t.status == 200)
        tableData.splice(0, tableData.length, ...t.data);
}
onMounted(() => {
    listQa();
})
const newQa = () => {
    qaData.id = null;
    qaData.question.question = '';
    qaData.similarQuestions = [];
    qaData.answer = ''
    dialogVisible.value = true
}
const showQaDetail = (idx) => {
    qaDetailIdx.value = idx
    const d = tableData[idx];
    if (d) {
        qaData.id = d.id;
        qaData.question.question = d.question.question;
        qaData.similarQuestions.splice(0, qaData.similarQuestions.length, ...d.similarQuestions);
        qaData.answer = d.answer
        qaDetailVisible.value = true
    }
}
const editQa = (idx) => {
    const d = tableData[idx];
    if (d) {
        qaData.id = d.id;
        qaData.question.question = d.question.question;
        qaData.similarQuestions.splice(0, qaData.similarQuestions.length, ...d.similarQuestions);
        qaData.answer = d.answer
        dialogVisible.value = true
    }
}
const saveQa = async () => {
    const t = await httpReq('POST', 'kb/qa', { robotId: robotId }, null, qaData);
    console.log(t);
    dialogVisible.value = false
    listQa()
}
const deleteQa = async (idx) => {
    ElMessageBox.confirm(
        'Confirm to delete this QnA?',
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
            const t = await httpReq('DELETE', 'kb/qa', { robotId: robotId }, null, qaData);
            console.log(t);
            nextTick(() => {
                qaDetailVisible.value = false
                listQa()
            })
        }
    }).catch(() => {
        // ElMessage({
        //     type: 'info',
        //     message: 'Delete canceled',
        // })
    })
}
const testQa = (text) => {
    loading.value = true;
    (async function (text) {
        const t = await httpReq('GET', 'kb/qa/dryrun', { robotId: robotId, text: text }, null, null);
        console.log(t);
        if (t.status == 200)
            testQnAResult.value = t.data[0].answer + ' (Distance: ' + t.data[1] + ')';
        else
            testQnAResult.value = t.err.message;
    })(text).then(() => loading.value = false);
}
const goBack = () => {
    router.push({ name: 'robotDetail', params: { robotId: robotId } });
}

const dialogVisible = ref(false)
const qaDetailVisible = ref(false)
const qaDetailIdx = ref(0)
const dryRunFormVisible = ref(false)
const loading = ref(false)
const testQnAText = ref('')
const testQnAResult = ref('')
const formLabelWidth = '120px'
</script>
<style scoped>
/* table {
    border-collapse: collapse;
    border-spacing: 0;
    border: none;
}

tr:nth-child(even) {
    background-color: #fff;
}

tr:nth-child(odd) {
    background-color: #f7f6f6;
}

tr:hover {
    background-color: #eee;
}

td {
    border: none;
    font-size: 15px;
} */
</style>
<template>
    <!-- <el-page-header :title="$t('lang.common.back')" @back="goBack">
        <template #content>
            <span class="text-large font-600 mr-3">Question and answers</span>
        </template>
</el-page-header>
<br /> -->
    <h1>Questions and answer</h1>
    <el-button type="primary" @click="newQa">Add QnA pair</el-button>
    <el-button type="primary" @click="dryRunFormVisible = true">Test QnA</el-button>
    <el-table :data="tableData" style="width: 100%">
        <el-table-column prop="question.question" label="Question" width="360" />
        <el-table-column prop="similarQuestions.length" label="No. of similar questions" width="180" />
        <el-table-column prop="answer" label="Answer" />
        <el-table-column fixed="right" label="Operations" width="200">
            <template #default="scope">
                <el-button link type="primary" @click="showQaDetail(scope.$index)">Detail</el-button>
                <el-button link type="primary" @click="editQa(scope.$index)">Edit</el-button>
                <el-button link type="danger" @click="deleteQa(scope.$index)">Delete</el-button>
            </template>
        </el-table-column>
    </el-table>
    <!-- <div v-for="(qa, index) in tableData" :id="index" :key="index">
        <el-divider />
        <table cellspacing="0">
            <tbody>
                <tr>
                    <td width="130">Question</td>
                    <td width="700">{{ qa.question.question }}</td>
                </tr>
                <tr>
                    <td valign="top">Similar questions</td>
                    <td>
                        <el-table :data="qa.similarQuestions" :show-header="false" style="width: 100%">
                            <el-table-column prop="question" label="" />
                        </el-table>
                    </td>
                </tr>
                <tr>
                    <td>Answer</td>
                    <td>{{ qa.answer }}</td>
                </tr>
                <tr>
                    <td></td>
                    <td>
                        <el-button type="primary" link @click="editQa(index)">
                            Edit
                        </el-button>
                        <el-button type="danger" link @click="deleteQa(index)">
                            Delete
                        </el-button>
                    </td>
                </tr>
            </tbody>
        </table>
    </div> -->
    <el-dialog v-model="dialogVisible" title="Add new QA" width="800">
        <el-form :model="qaData">
            <el-form-item label="Question" :label-width="formLabelWidth">
                <el-input v-model="qaData.question.question" placeholder="" />
            </el-form-item>
            <el-form-item v-for="(item, index) in qaData.similarQuestions" :id="index" :key="index"
                :label="index == 0 ? 'Similar questions' : ''" :label-width="formLabelWidth">
                <el-input v-model="qaData.similarQuestions[index].question" placeholder="" style="width: 90%;" />
                <el-button circle @click="qaData.similarQuestions.splice(index, 1)">-</el-button>
            </el-form-item>
            <el-form-item label="" :label-width="formLabelWidth">
                <el-button plain @click="qaData.similarQuestions.push({ question: '' })">Add similar
                    question</el-button>
            </el-form-item>
            <el-form-item label="Answer" :label-width="formLabelWidth">
                <el-input v-model="qaData.answer" placeholder="" type="textarea" :rows="5" />
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="saveQa">
                    {{ $t('lang.common.save') }}
                </el-button>
                <el-button @click="dialogVisible = false">Cancel</el-button>
            </div>
        </template>
    </el-dialog>
    <el-drawer v-model="qaDetailVisible" title="Detail of QnA" direction="rtl" size="50%">
        <el-form>
            <el-form-item label="Question" :label-width="formLabelWidth">
                {{ qaData.question.question }}
            </el-form-item>
            <el-form-item label="Similar questions" :label-width="formLabelWidth"
                v-show="qaData.similarQuestions.length > 0">
                <div v-for="(item, idx) in qaData.similarQuestions" :id="idx" :key="idx">
                    {{ item.question }}
                </div>
            </el-form-item>
            <el-form-item label="Answer" :label-width="formLabelWidth">
                {{ qaData.answer }}
            </el-form-item>
        </el-form>
        <div class="demo-drawer__footer">
            <el-button type="primary" :loading="loading" @click="dialogVisible = true">Edit</el-button>
            <el-button type="danger" :loading="loading" @click="deleteQa(qaDetailIdx)">Delete</el-button>
            <el-button @click="qaDetailVisible = false">Close</el-button>
        </div>
    </el-drawer>
    <el-drawer v-model="dryRunFormVisible" title="Test QnA" direction="rtl" size="50%">
        <el-form>
            <el-form-item label="">
                <el-input v-model="testQnAText" style="width: 240px" placeholder="Please input some texts" />
            </el-form-item>
            <el-form-item label="">
                {{ testQnAResult }}
            </el-form-item>
        </el-form>
        <div class="demo-drawer__footer">
            <el-button type="primary" :loading="loading" @click="testQa(testQnAText)">Test</el-button>
            <el-button @click="dryRunFormVisible = false">Close</el-button>
        </div>
    </el-drawer>
</template>