class Solution {
public:
    vector<int> survivedRobotsHealths(vector<int>& positions, vector<int>& healths, string directions) {
        int n = positions.size();
        unordered_map<int, pair<int, char>> mp;

        /// Instead of using unordered map use index array n sorted order...

        vector<int> sortedPos = positions;
        sort(sortedPos.begin(), sortedPos.end());

        for(int i = 0; i<n; i++){
            mp[positions[i]] = {healths[i], directions[i]};
        }

        stack<int> st;
        int prev, coming;
        
        for(int pos:sortedPos){
            if(st.empty()){
                st.push(pos);
                continue;
            }
            while(!st.empty()){
                prev = st.top();
                if(mp[pos].second == 'L' && mp[prev].second == 'R' && mp[pos].first > mp[prev].first){
                    mp[pos].first--;
                    st.pop();
                    mp.erase(prev);
                }
                else if(mp[pos].second == 'L' && mp[prev].second == 'R' && mp[pos].first < mp[prev].first){
                    mp.erase(pos);
                    mp[prev].first--;
                    break;
                }
                else if(mp[pos].second == 'L' && mp[prev].second == 'R' && mp[pos].first == mp[prev].first){
                    mp.erase(pos);
                    mp.erase(prev);
                    st.pop();
                    break;
                }
                else{
                    st.push(pos);
                    break;
                }
            }
        }

        vector<int> ans;

        for(int i = 0; i<n; i++){
            int pos = positions[i];
            if(mp.find(pos) != mp.end()){
                // cout<<pos<<endl;
                ans.push_back(mp[pos].first);
            }
        }

        return ans;
    }
};