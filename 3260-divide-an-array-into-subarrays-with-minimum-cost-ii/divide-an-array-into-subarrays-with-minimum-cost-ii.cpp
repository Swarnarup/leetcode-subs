class Solution {
public:
    long long minimumCost(vector<int>& nums, int k, int dist) {
        set<pair<int, int>> st1;     // {value , index}
        set<pair<int, int>> st2;     // {value , index}
        int n = nums.size();

        long long ans = 9223372036854775807;
        long long temp = 0;
        int i = 1;
        for(int j = 1; j<n; j++){
            if(st1.empty() || st1.size() < k-1){
                temp += nums[j];
                st1.insert({nums[j], j});
            }
            else if((--st1.end())->first > nums[j]){
                temp -= (--st1.end())->first;
                temp += nums[j];
                int val = (--st1.end())->first;
                int idx = (--st1.end())->second;
                st2.insert({val, idx});
                st1.erase({val, idx});
                st1.insert({nums[j], j});
            }
            else{
                st2.insert({nums[j], j});
            }

            if(j-i+1 == dist + 1){
                ans = min(ans, temp);

                if(st1.find({nums[i], i}) != st1.end()){
                    temp -= nums[i];
                    st1.erase({nums[i], i});
                    if(!st2.empty()){
                        int val = st2.begin()->first;
                        int idx = st2.begin()->second;
                        temp += val;
                        st1.insert({val, idx});
                        st2.erase({val, idx});
                    }
                }
                else if(st2.find({nums[i], i}) != st2.end()) st2.erase({nums[i], i});
                i++;
            }
        }

        return ans + nums[0];
    }
};