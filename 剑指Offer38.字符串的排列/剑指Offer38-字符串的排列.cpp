class Solution {
public:
    //此题思路同全排列Ⅰ，Ⅱ题（46，47题）
    vector<string> permutation(string s) {
        vector<string> res;
        dfs(s, res, 0);
        return res;
    }

    void dfs(string& s, vector<string>& res, int depth)
    {
        if(depth == s.size()-1)
        {
            res.push_back(s);
            return ;
        }
        unordered_set<char> used;  //局部set去重
        for(int i = depth; i < s.size(); ++i)
        {
            if(used.find(s[i]) != used.end())   continue;   //去重
            used.insert(s[i]);
            swap(s[depth],s[i]);
            dfs(s, res, depth+1);
            swap(s[depth],s[i]);        //回溯撤销操作
        }
    }
};