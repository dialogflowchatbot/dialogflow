<script setup>
import { checkConditionBranches, cloneObj, copyProperties, getDefaultBranch, httpReq } from '../../../assets/tools.js'
import { inject, reactive, ref, onMounted } from 'vue';
// import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n'
import EpPlus from '~icons/ep/plus'
import EpMinus from '~icons/ep/minus'
import EpWarning from '~icons/ep/warning'
const { t, tm, rt } = useI18n();
const lastTimeBranchIdMap = new Map();
const getNode = inject('getNode');
const { robotId } = inject('robotId');
const nodeSetFormVisible = ref(false);
const branchSetFormVisible = ref(false);
const formLabelWidth = '85px';
const defaultCondition = getDefaultBranch().conditionGroup[0][0];
defaultCondition.conditionType = '';
// defaultCondition.compareType = '';
// function resetDefaultCondition(c) {
//     c.refOptions = [];
//     c.compareOptions = [];
//     c.targetOptions = [];
//     c.inputVariable = false;
//     c.showCaseSensitiveCheckBox = false;
//     c.conditionType = '';
//     c.compareType = '';
//     c.refChoice = '';
//     c.targetValueVariant = 'Const';
//     c.targetValue = '';
// }
// resetDefaultCondition(defaultCondition);
const fallbackBranch = getDefaultBranch();
fallbackBranch.branchName = t('lang.common.else');
fallbackBranch.branchType = 'GotoAnotherNode';
fallbackBranch.editable = false;
// const defaultCondition = {
//     conditionType: '',
//     refChoice: '', compareType: '', targetChoice: '',
//     refOptions: [], compareOptions: [], targetOptions: [],
//     targetValue: '',
//     inputVariable: false,
// };
const defaultConditionGroup = [[]];
const types18 = tm('lang.conditionNode.types')
const conditionTypes = [
    { label: types18[0], value: 'UserIntent' },
    // { label: 'Zero-Shot Classification', value: 'ZeroShotTextClassification' },
    { label: types18[1], value: 'UserInput' },
    { label: types18[2], value: 'FlowVariable' },
];
const refOptionsSet = {
    "FlowVariable": [],
};
const compares18 = tm('lang.conditionNode.compares')
const compareOptionsSet = {
    "UserIntent": [{ label: compares18[0], value: 'Eq', inputType: 0, showCS: false }],
    "UserInput": [
        { label: compares18[0], value: 'Eq', inputType: 1, showCS: true },
        { label: compares18[2], value: 'Contains', inputType: 1, showCS: true },
        { label: compares18[3], value: 'Timeout', inputType: 0, showCS: false }
    ],
    "FlowVariable": [
        { label: 'Has value', value: 'HasValue', inputType: 0, showCS: false, belongsTo: 'StrNum' },
        { label: 'Does not have value', value: 'DoesNotHaveValue', inputType: 0, showCS: false, belongsTo: 'StrNum' },
        { label: 'Is empty string', value: 'EmptyString', inputType: 0, showCS: false, belongsTo: 'Str' },
        { label: compares18[0], value: 'Eq', inputType: 1, showCS: true, belongsTo: 'StrNum' },
        { label: compares18[1], value: 'NotEq', inputType: 1, showCS: true, belongsTo: 'StrNum' },
        { label: 'Contains', value: 'Contains', inputType: 1, showCS: true, belongsTo: 'Str' },
        { label: 'Not contains', value: 'NotContains', inputType: 1, showCS: true, belongsTo: 'Str' },
        { label: 'Greater than', value: 'NGT', inputType: 1, showCS: false, belongsTo: 'Num' },
        { label: 'Greater than or equal to', value: 'NGTE', inputType: 1, showCS: false, belongsTo: 'Num' },
        { label: 'Less than', value: 'NLT', inputType: 1, showCS: false, belongsTo: 'Num' },
        { label: 'Less than or equal to', value: 'NLTE', inputType: 1, showCS: false, belongsTo: 'Num' },
    ],
    "ZeroShotTextClassification": []
};
const targetOptionsSet = {
    "UserIntent": [],
};
const loading = false;
let editConditionIndex = -1;
const nodeData = reactive({
    nodeName: t('lang.conditionNode.nodeName'),
    branches: [fallbackBranch],
    valid: false,
    invalidMessages: [],
    newNode: true,
});
const _branch = getDefaultBranch();
_branch.conditionGroup = [];
const branch = reactive(_branch);
// const branch = reactive({
//     branchId: '',
//     branchName: '',
//     branchType: 'Condition',
//     conditionGroup: [],
// });
const nodeName = ref();

getNode().on("change:data", ({ current }) => {
    nodeSetFormVisible.value = true;
});
defaultConditionGroup[0].push(cloneObj(defaultCondition));

onMounted(async () => {
    // console.log('conditionNode')
    let t = await httpReq('GET', 'intent', { robotId: robotId }, null, null);
    // console.log(t);
    if (t && t.status == 200 && t.data) {
        const d = targetOptionsSet.UserIntent;
        d.splice(0, d.length);
        t.data.forEach(function (item, index, arr) {
            // console.log(item.name)
            this.push({ label: item.intent_name, value: item.intent_name });
        }, d);
    }
    t = await httpReq('GET', 'variable', { robotId: robotId }, null, null);
    // console.log(t);
    if (t && t.status == 200 && t.data) {
        const d = refOptionsSet.FlowVariable;
        d.splice(0, d.length);
        t.data.forEach(function (item, index, arr) {
            this.push({ label: item.varName, value: item.varName, vtype: item.varType });
        }, d);
    }

    const node = getNode();
    const data = node.getData();
    console.log(data);
    copyProperties(data, nodeData);
    // console.log(nodeData);
    // if (data) {
    //     if (data.nodeName)
    //         nodeData.nodeName = data.nodeName;
    // if (data.branches)
    //     nodeData.branches = data.branches;
    //     console.log(nodeData.branches);
    // }
    if (nodeData.newNode)
        nodeData.nodeName += data.nodeCnt.toString();
    else {
        nodeData.branches.forEach(function (b, idx) {
            lastTimeBranchIdMap.set(b.branchId, idx)
        })
    }
    nodeData.newNode = false;
    validate();
});
const errors18 = tm('lang.conditionNode.errors')
const validateBranchName = (rule, value, callback) => {
    if (value == '')
        callback(new Error(errors18[0]));
    else {
        for (let i = 0; i < nodeData.branches.length; i++) {
            if (nodeData.branches[i].branchName == value) {
                callback(new Error(errors18[1]));
                return;
            }
        }
        callback();
    }
};
const branchValidators = reactive({
    branchName: [{ validator: validateBranchName, trigger: 'blur' }],
});

function validate() {
    const d = nodeData;
    const m = d.invalidMessages;
    m.splice(0, m.length);
    if (!d.nodeName)
        m.push(errors18[2]);
    if (d.branches.length == 0)
        m.push(errors18[3]);
    const ret = checkConditionBranches(d.branches);
    console.log(ret)
    if (!ret.r)
        m.push(ret.m);
    d.valid = m.length == 0;
}
function addNewBranch() {
    // console.log(branch);
    editConditionIndex = -1;
    branch.branchName = '';
    branch.conditionGroup = cloneObj(defaultConditionGroup);
    // resetDefaultCondition(branch.conditionGroup[0][0]);
    branchSetFormVisible.value = true;
}
function hideBranchForm() {
    branchSetFormVisible.value = false;
}
function saveBranch() {
    if (!branch.branchName) {
        ElMessage({
            message: errors18[0],
            type: 'warning',
        });
        return;
    }
    if (editConditionIndex == -1)
        nodeData.branches.unshift(cloneObj(branch));
    else {
        nodeData.branches[editConditionIndex].branchName = branch.branchName;
        console.log(branch.conditionGroup);
        nodeData.branches[editConditionIndex].conditionGroup = cloneObj(branch.conditionGroup);
    }
    hideBranchForm();
}
function editBranch(i) {
    // branch = cloneObj(nodeData.branches[i]);
    // branch = nodeData.branches[i];
    branch.branchName = nodeData.branches[i].branchName;
    branch.conditionGroup = cloneObj(nodeData.branches[i].conditionGroup);
    editConditionIndex = i;
    branchSetFormVisible.value = true;
}
function removeBranch(i) {
    ElMessageBox.confirm(
        'Do you delete this branch?',
        'Warning',
        {
            confirmButtonText: t('lang.common.del'),
            cancelButtonText: t('lang.common.cancel'),
            type: 'warning',
        }
    ).then(() => {
        nodeData.branches.splice(i, 1);
        hideBranchForm();
    }).catch(() => {
        // ElMessage({
        //     type: 'info',
        //     message: 'Delete canceled',
        // })
    })
}
function hideForm() {
    // const { nodeSetFormVisible } = .getNode().getData();
    // console.log(getNode().getData());
    nodeSetFormVisible.value = false;
}
function saveForm() {
    let heightOffset = nodeName.value.offsetHeight;
    const node = getNode();
    const len = nodeData.branches.length;
    // let hasFallbackBranch = false;
    // for (let i = 0; i < len; i++) {
    //     if (branch.branchType == 'GotoAnotherNode') {
    //         hasFallbackBranch = true;
    //     }
    // }
    // if (!hasFallbackBranch)
    //     nodeData.branches.push(fallbackBranch);
    const x = nodeName.value.offsetWidth - 15;
    // node.removePorts();
    let port, id;
    const newBranchIdMap = new Map()
    // console.log(node)
    for (let i = 0; i < len; i++) {
        heightOffset += 20;
        id = nodeData.branches[i].branchId
        // console.log(nodeData.branches[i].branchName + ' ' + heightOffset)
        if (id) {
            // console.log(nodeData.branches[i].branchName + ' has id ' + id)
            lastTimeBranchIdMap.delete(id)
            port = node.getPort(id)
            // console.log('n2=' + JSON.stringify(port))
            if (port)
                node.setPortProp(port.id, ['args', 'y'], heightOffset);
        } else {
            // console.log(nodeData.branches[i].branchName + ' dose not have id ')
            // nodeData.branches[i].branchId = Math.random().toString(36).substring(2);
            node.addPort({
                // id: nodeData.branches[i].branchId,
                group: 'absolute',
                args: { x: x, y: heightOffset },
                attrs: {
                    text: {
                        text: nodeData.branches[i].branchName,
                        fontSize: 12,
                    },
                },

            }, { silent: false });
            // console.log(node.ports)
            // console.log(node.ports.items[node.ports.items.length - 1].id)
            nodeData.branches[i].branchId = node.ports.items[node.ports.items.length - 1].id;
        }
        newBranchIdMap.set(nodeData.branches[i].branchId, i)
        // console.log(nodeData.branches[i].branchId);
        if (branch.branchType == 'GotoAnotherNode') {
            hasFallbackBranch = true;
        }
    }
    lastTimeBranchIdMap.forEach((value, key, map) => {
        // console.log('remove ' + key)
        node.removePort(key, { silent: false })
        lastTimeBranchIdMap.delete(key)
    })
    for (const [key, value] of newBranchIdMap) {
        lastTimeBranchIdMap.set(key, value);
    }
    node.resize(node.size().width, 20 + heightOffset, { direction: 'bottom', silent: false })
    validate();
    node.removeData({ silent: true });
    node.setData(nodeData, { silent: true });
    hideForm();
    // console.log(node.getData());
}
function showOptions(v, groupIndex, conditionIdx) {
    console.log(v + ' ' + groupIndex + ' ' + conditionIdx);
    const condition = branch.conditionGroup[groupIndex][conditionIdx];
    // condition.refChoice = '';
    // condition.compareType = '';
    // condition.targetValueVariant = 'Const';
    // condition.targetValue = '';
    // condition.inputVariable = false;
    const refOptions = refOptionsSet[v];
    if (refOptions)
        condition.refOptions = refOptions;
    else
        condition.refOptions = [];
    // compareOptions.splice(0, compareOptions.length);
    condition.compareOptions = compareOptionsSet[v];
    condition.compareType = '';
    // if (condition.compareOptions.length == 1)
    //     condition.compareType = condition.compareOptions[0].value;
    const targetOptions = targetOptionsSet[v];
    console.log(targetOptions);
    if (targetOptions)
        condition.targetOptions = targetOptions;
    else
        condition.targetOptions = [];
}
function percolateCompareOptions(compareType, groupIndex, conditionIdx, refOption) {
    const condition = branch.conditionGroup[groupIndex][conditionIdx];
    if (condition.compareOptions[0].belongsTo) {
        const selectedVar = refOptionsSet.FlowVariable.filter(function (curValue, index, arr) {
            return curValue.label === refOption;
        });
        // console.log(selectedVar);
        if (selectedVar.length == 1) {
            condition.compareType = '';
            condition.compareOptions = compareOptionsSet[compareType].filter(function (curVar, index, arr) {
                // console.log(curVar.belongsTo);
                // console.log(selectedVar[0].vtype);
                // console.log(curVar.belongsTo.indexOf(selectedVar[0].vtype));
                return curVar.belongsTo.indexOf(selectedVar[0].vtype) > -1;
            });
        }
    }
}
function selectCompareOption(groupIndex, conditionIndex, item) {
    // console.log(item.inputType);
    branch.conditionGroup[groupIndex][conditionIndex].inputVariable = item.inputType == 1;
    branch.conditionGroup[groupIndex][conditionIndex].showCaseSensitiveCheckBox = item.showCS;
}
function addContidion(conditionGroup) {
    const cond = cloneObj(defaultCondition);
    // resetDefaultCondition(cond);
    conditionGroup.push(cond);
}
function addConditionGroup() {
    // console.log(branch);
    branch.conditionGroup.push(...cloneObj(defaultConditionGroup));
    // console.log(branch);
}
function removeConditionGroup(groupIdx) {
    branch.conditionGroup.splice(groupIdx, 1);
}
</script>
<style scoped>
.nodeBox {
    border: 2px #0000000e solid;
    height: 100%;
    width: 100%;
    background-color: white;
}

.nodeTitle {
    background-color: rgb(145, 113, 227);
    color: white;
    font-weight: 500;
    font-size: 0.9rem;
    padding: 5px;
}

.optionWidth {
    width: 130px;
}
</style>
<template>
    <div class="nodeBox">
        <div ref="nodeName" class="nodeTitle">
            {{ nodeData.nodeName }}
            <span v-show="nodeData.invalidMessages.length > 0">
                <el-tooltip class="box-item" effect="dark" :content="nodeData.invalidMessages.join('<br/>')"
                    placement="bottom" raw-content>
                    <el-icon color="red" size="16">
                        <EpWarning />
                    </el-icon>
                </el-tooltip>
            </span>
        </div>
        <!-- <teleport to="body"> -->
        <el-dialog v-model="branchSetFormVisible" :title="t('lang.conditionNode.newBranch')" width="75%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :model="branch" :rules="branchValidators">
                <el-form-item :label="t('lang.conditionNode.condName')" :label-width="formLabelWidth" prop="branchName">
                    <el-input v-model="branch.branchName" autocomplete="off" minlength="1" maxlength="15" />
                </el-form-item>
                <el-form-item v-for="(g, groupIndex) in branch.conditionGroup" :key="groupIndex"
                    :label="t('lang.conditionNode.condType')" :label-width="formLabelWidth">
                    <div v-for="(c, index) in g" :key="index">
                        <el-select v-model="c.conditionType" :placeholder="t('lang.conditionNode.condTypePH')"
                            @change="(v) => showOptions(v, groupIndex, index)" style="width:116px;">
                            <el-option v-for="item in conditionTypes" :key="item.label" :label="item.label"
                                :value="item.value" />
                        </el-select>
                        <el-select v-model="c.refChoice" :placeholder="t('lang.conditionNode.comparedPH')"
                            v-show="c.refOptions.length > 0" class="optionWidth"
                            @change="(v) => percolateCompareOptions(c.conditionType, groupIndex, index, v)">
                            <el-option v-for="item in c.refOptions" :key="item.label" :label="item.label"
                                :value="item.value" />
                        </el-select>
                        <el-select v-model="c.compareType" :placeholder="t('lang.conditionNode.compareTypePH')"
                            v-show="c.compareOptions.length > 0" class="optionWidth">
                            <el-option v-for="item in c.compareOptions" :key="item.label" :label="item.label"
                                :value="item.value" @click.native="selectCompareOption(groupIndex, index, item)" />
                        </el-select>
                        <el-select v-model="c.targetValue" :placeholder="t('lang.conditionNode.targetPH')"
                            v-show="c.targetOptions.length > 0" class="optionWidth">
                            <el-option v-for="item in c.targetOptions" :key="item.label" :label="item.label"
                                :value="item.value" />
                        </el-select>
                        <el-select v-model="c.targetValueVariant" v-show="c.inputVariable" class="optionWidth">
                            <el-option label="const value" value="Const" />
                            <el-option label="variable value" value="Variable" />
                        </el-select>
                        <el-select v-model="c.targetValue" placeholder="Please choose a variable"
                            v-show="c.inputVariable && c.targetValueVariant == 'Variable'" class="optionWidth">
                            <el-option v-for="item in refOptionsSet['FlowVariable']" :key="item.label"
                                :label="item.label" :value="item.value" />
                        </el-select>
                        <el-input v-model="c.targetValue" class="optionWidth"
                            v-show="c.inputVariable && c.targetValueVariant == 'Const'" />
                        <el-checkbox v-show="c.showCaseSensitiveCheckBox" v-model="c.caseSensitiveComparison"
                            label="CaseSensitive" />
                        <el-button-group>
                            <el-button type="primary" @click="addContidion(g)">
                                <el-icon>
                                    <EpPlus />
                                </el-icon>
                                <!-- {{ t('lang.conditionNode.andCond') }} -->
                            </el-button>
                            <el-button type="danger" v-show="g.length > 1" @click="g.splice(index, 1); console.log(g)">
                                <el-icon>
                                    <EpMinus />
                                </el-icon>
                            </el-button>
                        </el-button-group>
                        <el-divider border-style="dashed" />
                    </div>
                    <el-divider />
                    <el-button type="danger" v-show="branch.conditionGroup.length > 1"
                        @click="removeConditionGroup(groupIndex)">
                        X
                    </el-button>
                </el-form-item>
                <el-form-item label="" :label-width="formLabelWidth">
                    <el-button type="primary" @click="addConditionGroup()">
                        <el-icon>
                            <EpPlus />
                        </el-icon>
                        <!-- {{ t('lang.conditionNode.orCond') }} -->
                    </el-button>
                </el-form-item>
            </el-form>
            <template #footer>
                <span class="dialog-footer">
                    <el-button type="primary" @click="saveBranch()">
                        {{ t('lang.common.save') }}
                    </el-button>
                    <el-button v-on:click="hideBranchForm()">{{ t('lang.common.cancel') }}</el-button>
                    <el-button type="danger" :disabled="editConditionIndex == -1"
                        v-on:click="removeBranch(editConditionIndex)">{{ t('lang.common.del') }}</el-button>
                </span>
            </template>
        </el-dialog>
        <el-drawer v-model="nodeSetFormVisible" :title="nodeData.nodeName" direction="rtl" size="70%"
            :append-to-body="true" :destroy-on-close="true">
            <el-form :model="nodeData">
                <el-form-item :label="t('lang.common.nodeName')" :label-width="formLabelWidth">
                    <el-input v-model="nodeData.nodeName" autocomplete="off" />
                </el-form-item>
                <el-form-item :label="t('lang.conditionNode.newCond')" :label-width="formLabelWidth">
                    <el-button type="primary" @click="addNewBranch()">
                        <el-icon>
                            <EpPlus />
                        </el-icon>
                    </el-button>
                    <el-button v-for="(btn, index) in nodeData.branches" :key="index" type="primary"
                        @click="editBranch(index)" :disabled="!btn.editable">{{ btn.branchName }}</el-button>
                </el-form-item>
            </el-form>
            <div>
                <el-button type="primary" :loading="loading" @click="saveForm()">{{ t('lang.common.save') }}</el-button>
                <el-button @click="hideForm()">{{ t('lang.common.cancel') }}</el-button>
            </div>
        </el-drawer>
        <!-- </teleport> -->
    </div>
</template>